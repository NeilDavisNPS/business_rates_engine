//! Suite A: Canonical Multi-Jurisdiction Scenario Fixture Tests (Scenarios 1–11)
//!
//! Part of the Business Rates Engine statutory test suite.

#[allow(unused_imports)]
use super::common::*;
#[allow(unused_imports)]
use rust_decimal_macros::dec;
#[allow(unused_imports)]
use chrono::NaiveDate;
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use axum::body::Body;
#[allow(unused_imports)]
use axum::http::{header, Request, StatusCode, HeaderMap};
#[allow(unused_imports)]
use tower::ServiceExt;

#[allow(unused_imports)]
use business_rates_engine::models::calculation::*;
#[allow(unused_imports)]
use business_rates_engine::models::enums::{Jurisdiction, PartyType, OccupancyType};
#[allow(unused_imports)]
use business_rates_engine::engine::calculator::CalculationEngine;
#[allow(unused_imports)]
use business_rates_engine::db::repository::ParameterRepository;
#[allow(unused_imports)]
use business_rates_engine::models::AppState;
#[allow(unused_imports)]
use business_rates_engine::api::handlers::resolve_language;
#[allow(unused_imports)]
use business_rates_engine::i18n::Language;
#[allow(unused_imports)]
use business_rates_engine::error::AppError;

#[tokio::test]
async fn test_scenario_1_england_sbrr_and_rhl_multi_year() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_1_sbrr_rhl_england.json");

    let response = engine.calculate(request);

    assert_eq!(response.jurisdiction, Jurisdiction::ENGLAND);
    assert_eq!(response.annual_billing_segments.len(), 2);
    assert_eq!(response.parameter_version_ids.param_multiplier_version, "v2026.1");

    let seg1 = &response.annual_billing_segments[0];
    assert_eq!(seg1.financial_year, "2025/26");
    assert_eq!(seg1.rateable_value, dec!(14500.00));
    assert_eq!(seg1.multiplier_used.rate, dec!(0.499));
    assert!(seg1.itemized_reliefs.iter().any(|r| r.relief_code == "SBRR"));

    let seg2 = &response.annual_billing_segments[1];
    assert_eq!(seg2.financial_year, "2026/27");
    assert_eq!(seg2.rateable_value, dec!(16000.00));
    assert_eq!(seg2.multiplier_used.rate, dec!(0.382));
    assert_eq!(seg2.multiplier_used.multiplier_type, "SMALL_BUSINESS_RHL");
    // In England 2026/27, temporary RHL relief has ceased on 31 March 2026, replaced by permanent lower multiplier (38.2p)
    assert!(!seg2.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")));

    assert!(response.calculation_narrative.contains("Local Government Finance Act 1988"));
}

#[tokio::test]
async fn test_scenario_2_wales_petrol_station() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_2_wales_petrol_station.json");

    let response = engine.calculate(request);
    assert_eq!(response.jurisdiction, Jurisdiction::WALES);
    assert_eq!(response.annual_billing_segments[0].multiplier_used.rate, dec!(0.502));
    assert_eq!(response.annual_billing_segments[0].multiplier_used.multiplier_type, "STANDARD");
}

#[tokio::test]
async fn test_scenario_3_scotland_self_catering() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_3_scotland_self_catering.json");

    let response = engine.calculate(request);
    assert_eq!(response.jurisdiction, Jurisdiction::SCOTLAND);
    assert_eq!(response.annual_billing_segments[0].net_segment_payable, dec!(0.00));
    assert!(response.annual_billing_segments[0].itemized_reliefs.iter().any(|r| r.relief_code == "SBBS"));
    // Suite 10: Holiday Let Borderline Verification
    assert!(response.calculation_narrative.contains("SCAT 249 Holiday Accommodation"));
}

#[tokio::test]
async fn test_scenario_4_welsh_artificial_avoidance() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_4_welsh_artificial_avoidance.json");

    let response = engine.calculate(request);
    assert_eq!(response.jurisdiction, Jurisdiction::WALES);
    assert_eq!(response.annual_billing_segments[0].rateable_value, dec!(85000.00));

    // Suite 9: Welsh WSI 2026/43 Civil Penalty Assessment (£500 + 3% of RV)
    let penalties = &response.annual_billing_segments[0].statutory_penalties;
    assert_eq!(penalties.len(), 1);
    assert_eq!(penalties[0].penalty_type, "WELSH_ARTIFICIAL_AVOIDANCE");
    // £500 + (85000 * 0.03) = £500 + £2550 = £3050
    assert_eq!(penalties[0].amount, dec!(3050.00));
    assert_eq!(response.summary_financials.total_statutory_penalties, Some(dec!(3050.00)));
}

#[tokio::test]
async fn test_scenario_5_airport_high_value() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_5_airport_high_value.json");

    let response = engine.calculate(request);
    assert_eq!(response.jurisdiction, Jurisdiction::ENGLAND);
    assert_eq!(response.annual_billing_segments[0].multiplier_used.multiplier_type, "HIGH_VALUE");
    assert_eq!(response.annual_billing_segments[0].multiplier_used.rate, dec!(0.508));
}

#[tokio::test]
async fn test_mandatory_charitable_relief_80_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_6_charity_mandatory_80.json");

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let charity_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(charity_relief.discount_percentage, dec!(80.00));
}

#[tokio::test]
async fn test_multi_occupancy_periods_single_liable_party() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // Liable party with 3 distinct episodes across 2024/25:
    // 1. UNOCCUPIED: 2024-04-01 to 2024-06-30 (91 days) -> gets 100% empty exemption (3-month rate-free period)
    // 2. OCCUPIED: 2024-07-01 to 2024-12-31 (184 days) -> 100% SBRR; consecutive days = 184 >= 91 resets empty allowance
    // 3. PART_OCCUPIED: 2025-01-01 to 2025-03-31 (90 days) -> 50% occupied (£6,000) & 50% unoccupied (£6,000)
    //    Because empty allowance reset, unoccupied gets s.44A relief; occupied gets SBRR!
    let request = load_fixture("scenario_7_multi_occupancy_episodes.json");

    let response = engine.calculate(request);
    assert_eq!(response.annual_billing_segments.len(), 3);

    // Segment 1: UNOCCUPIED (91 days)
    let seg1 = &response.annual_billing_segments[0];
    assert_eq!(seg1.occupancy_type, Some(OccupancyType::UNOCCUPIED));
    assert_eq!(seg1.segment_period.days, 91);
    let empty_relief = seg1.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION");
    assert!(empty_relief.is_some(), "Segment 1 should have empty property relief");
    assert_eq!(seg1.net_segment_payable, dec!(0.00));

    // Segment 2: OCCUPIED (184 days)
    let seg2 = &response.annual_billing_segments[1];
    assert_eq!(seg2.occupancy_type, Some(OccupancyType::OCCUPIED));
    assert_eq!(seg2.segment_period.days, 184);
    let sbrr = seg2.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR");
    assert!(sbrr.is_some(), "Segment 2 should have SBRR");
    assert_eq!(seg2.net_segment_payable, dec!(0.00));

    // Segment 3: PART_OCCUPIED (90 days)
    let seg3 = &response.annual_billing_segments[2];
    assert_eq!(seg3.occupancy_type, Some(OccupancyType::PART_OCCUPIED));
    assert_eq!(seg3.segment_period.days, 90);
    let s44a = seg3.itemized_reliefs.iter().find(|r| r.relief_code == "SECTION_44A_PART_OCCUPIED");
    assert!(s44a.is_some(), "Segment 3 should have s.44A part-occupied relief");
    let occ_sbrr = seg3.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR");
    assert!(occ_sbrr.is_some(), "Segment 3 should have SBRR on occupied portion");
    assert_eq!(seg3.net_segment_payable, dec!(0.00));

    // Total net payable across all 3 episodes
    assert_eq!(response.summary_financials.total_net_payable, dec!(0.00));
    assert!(response.calculation_narrative.contains("Occupied (Full Occupation)"));
    assert!(response.calculation_narrative.contains("Unoccupied (Empty Property)"));
    assert!(response.calculation_narrative.contains("Part-Occupied (Section 44A)"));
}

#[tokio::test]
async fn test_suite_8_empty_property_exemption() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Industrial warehouse (SCAT 096) unoccupied for 90 days (within 182-day exemption)
    let request = load_fixture("scenario_8_empty_property_exemption.json");

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let empty_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION");
    assert!(empty_relief.is_some());
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_ndra_2023_improvement_relief_g_factor() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_9_improvement_relief_g_factor.json");

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let g_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "IMPROVEMENT_G_FACTOR").unwrap();
    // 5000 * 0.499 = 2495.00
    assert_eq!(g_relief.amount_deducted, dec!(2495.00));
}

#[tokio::test]
async fn test_statutory_10_and_12_installments_schedule() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: Default 10 monthly statutory installments (SI 1989/1058 Schedule 1)
    let mut req10 = load_fixture("scenario_10_statutory_12_installments.json");
    req10.qualifying_relief_flags.payment_installments_count = None; // Defaults to 10
    let res10 = engine.calculate(req10);
    let seg10 = &res10.annual_billing_segments[0];
    let inst10 = seg10.statutory_installments.as_ref().unwrap();

    assert_eq!(inst10.len(), 10);
    assert_eq!(inst10[0].due_date, NaiveDate::from_ymd_opt(2026, 5, 1).unwrap());
    assert_eq!(inst10[9].due_date, NaiveDate::from_ymd_opt(2027, 2, 1).unwrap());
    let total10: rust_decimal::Decimal = inst10.iter().map(|i| i.amount).sum();
    assert_eq!(total10, seg10.net_segment_payable);

    // Case 2: 12 monthly installments (Local Government Act 2003 / SI 2014/466)
    let req12 = load_fixture("scenario_10_statutory_12_installments.json");
    let res12 = engine.calculate(req12);
    let seg12 = &res12.annual_billing_segments[0];
    let inst12 = seg12.statutory_installments.as_ref().unwrap();

    assert_eq!(inst12.len(), 12);
    assert_eq!(inst12[0].due_date, NaiveDate::from_ymd_opt(2026, 4, 1).unwrap());
    assert_eq!(inst12[11].due_date, NaiveDate::from_ymd_opt(2027, 3, 1).unwrap());
    let total12: rust_decimal::Decimal = inst12.iter().map(|i| i.amount).sum();
    assert_eq!(total12, seg12.net_segment_payable);
}

#[tokio::test]
async fn test_statutory_rural_settlement_relief() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_11_rural_settlement_relief.json");

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let rural_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "RURAL_SETTLEMENT");
    assert!(rural_relief.is_some());
    let r = rural_relief.unwrap();
    assert_eq!(r.statutory_authority, "LGFA 1988 s.43(6B) / LGFA 1997");
    assert_eq!(r.discount_percentage, dec!(100.00));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}
