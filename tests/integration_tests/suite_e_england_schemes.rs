//! Suite E: English Multipliers, Transitional Phasing, Grace Periods & SSBR
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
async fn test_england_2026_five_multipliers_rhl_vs_non_rhl() {
    let repo = ParameterRepository::new_seeded();

    // 1. Small Business RHL (< £51k with RHL)
    let m1 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(30000.00), true, true, false, false).unwrap();
    assert_eq!(m1.rate, dec!(0.382));
    assert_eq!(m1.multiplier_type, "SMALL_BUSINESS_RHL");

    // 2. Small Business non-RHL (< £51k without RHL)
    let m2 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(30000.00), false, false, false, false).unwrap();
    assert_eq!(m2.rate, dec!(0.432));
    assert_eq!(m2.multiplier_type, "SMALL_BUSINESS");

    // 3. Standard RHL (£51k <= RV < £500k with RHL)
    let m3 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(100000.00), true, true, false, false).unwrap();
    assert_eq!(m3.rate, dec!(0.430));
    assert_eq!(m3.multiplier_type, "STANDARD_RHL");

    // 4. Standard non-RHL (£51k <= RV < £500k without RHL)
    let m4 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(100000.00), false, false, false, false).unwrap();
    assert_eq!(m4.rate, dec!(0.480));
    assert_eq!(m4.multiplier_type, "STANDARD");

    // 5. High-Value (RV >= £500k)
    let m5 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(600000.00), false, false, false, false).unwrap();
    assert_eq!(m5.rate, dec!(0.508));
    assert_eq!(m5.multiplier_type, "HIGH_VALUE");
}

#[tokio::test]
async fn test_england_sbrr_second_property_grace_period() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(10000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: false,
            is_small_business_second_property_grace: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let sbrr = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR");
    assert!(sbrr.is_some(), "SBRR should be granted during 12-month grace period on secondary property acquisition");
    assert_eq!(sbrr.unwrap().discount_percentage, dec!(100.00));
    assert!(sbrr.unwrap().statutory_authority.contains("S.I. 2017/411 art. 3(4) (12-Month Grace Period)"));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_statutory_sbrr_england_linear_taper() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // RV £13,500: halfway between £12,000 (100% relief) and £15,000 (0% relief)
    // Linear formula: (15,000 - 13,500) / (15,000 - 12,000) * 100% = 1,500 / 3,000 * 100% = 50.00%
    let request = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(13500.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    // English small business multiplier for 2025/26 is 0.499
    // Gross = 13500 * 0.499 = 6736.50
    assert_eq!(seg.rateable_value, dec!(13500.00));
    assert_eq!(seg.gross_liability, dec!(6736.50));

    let sbrr_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(sbrr_relief.discount_percentage, dec!(50.00));
    assert_eq!(sbrr_relief.amount_deducted, dec!(3368.25));
    assert_eq!(seg.net_segment_payable, dec!(3368.25));
}

#[tokio::test]
async fn test_suite_7_transitional_phasing_relief_cap() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2020, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2023, 3, 31).unwrap()),
                    rateable_value: dec!(10000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(30000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    // Capped under 2023 Revaluation List Medium property cap (+15% for £20k < RV <= £100k)
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(9231.50));
    assert_eq!(response.summary_financials.total_transitional_adjustments, dec!(9231.50));
    assert!(response.calculation_narrative.contains("Transitional Adjustment [RELIEF]"));
}

#[tokio::test]
async fn test_suite_7_transitional_relief_none_when_under_cap() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2020, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2023, 3, 31).unwrap()),
                    rateable_value: dec!(20000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(20500.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.transitional_adjustment.adjustment_type, "NONE");
    assert_eq!(seg.transitional_adjustment.amount, dec!(0.00));
    assert_eq!(response.summary_financials.total_transitional_adjustments, dec!(0.00));
}

#[tokio::test]
async fn test_england_transitional_relief_opt_out() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let base_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2026, 3, 31).unwrap()),
                    rateable_value: dec!(10000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(16000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(false),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res = engine.calculate(base_req);
    let seg = &res.annual_billing_segments[0];

    assert_eq!(seg.transitional_adjustment.adjustment_type, "NONE");
    assert_eq!(seg.transitional_adjustment.amount, dec!(0.00));
    // Gross unphased: £16,000 * 0.432 = £6,912.00
    assert_eq!(seg.gross_liability, dec!(6912.00));
    assert_eq!(seg.net_segment_payable, dec!(6912.00));
}

#[test]
fn test_english_transitional_relief_compounding_across_multiple_years() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2017-04-01",
                    "effective_to": "2023-03-31",
                    "rateable_value": 20000.00
                },
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2023-04-01",
            "end_date": "2026-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {}
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json.clone()).unwrap();
    let response = engine.calculate(request);

    assert_eq!(response.annual_billing_segments.len(), 3);

    // Year 1 (2023/24): Base = £20,000 * 0.499 = £9,980. Capped (+15%) = £11,477. TR = £19,960 - £11,477 = £8,483.
    let seg1 = &response.annual_billing_segments[0];
    assert_eq!(seg1.financial_year, "2023/24");
    assert_eq!(seg1.gross_liability, dec!(19960.00));
    assert_eq!(seg1.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg1.transitional_adjustment.amount, dec!(8483.00));
    assert_eq!(seg1.net_segment_payable, dec!(11477.00));

    // Year 2 (2024/25): Base = £11,477. Capped (+25%) = £14,346.25. TR = £19,960 - £14,346.25 = £5,613.75.
    let seg2 = &response.annual_billing_segments[1];
    assert_eq!(seg2.financial_year, "2024/25");
    assert_eq!(seg2.gross_liability, dec!(19960.00));
    assert_eq!(seg2.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg2.transitional_adjustment.amount, dec!(5613.75));
    assert_eq!(seg2.net_segment_payable, dec!(14346.25));

    // Year 3 (2025/26): Base = £14,346.25. Capped (+40%) = £20,084.75. Gross £19,960 < £20,084.75 -> TR = £0.
    let seg3 = &response.annual_billing_segments[2];
    assert_eq!(seg3.financial_year, "2025/26");
    assert_eq!(seg3.gross_liability, dec!(19960.00));
    assert_eq!(seg3.transitional_adjustment.amount, dec!(0.00));
    assert_eq!(seg3.net_segment_payable, dec!(19960.00));

    // Standalone Year 2 (2024/25) calculation must yield the exact same compounded transitional relief
    let mut json_y2 = json.clone();
    json_y2["occupation_period"] = serde_json::json!({
        "start_date": "2024-04-01",
        "end_date": "2025-03-31"
    });
    let request_y2: LiabilityCalculationRequest = serde_json::from_value(json_y2).unwrap();
    let response_y2 = engine.calculate(request_y2);
    assert_eq!(response_y2.annual_billing_segments.len(), 1);
    let seg_y2 = &response_y2.annual_billing_segments[0];
    assert_eq!(seg_y2.transitional_adjustment.amount, dec!(5613.75));
    assert_eq!(seg_y2.net_segment_payable, dec!(14346.25));
}

#[test]
fn test_transitional_relief_preserved_when_occupation_starts_partway_through_year() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2017-04-01",
                    "effective_to": "2023-03-31",
                    "rateable_value": 20000.00
                },
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2023-06-01",
            "end_date": "2024-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {}
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);

    assert_eq!(response.annual_billing_segments.len(), 1);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(16633.33));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(7069.16));
    assert_eq!(seg.net_segment_payable, dec!(9564.17));
}

#[tokio::test]
async fn test_england_supporting_small_business_relief_2023_cap_600() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2020, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2023, 3, 31).unwrap()),
                    rateable_value: dec!(10000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(16000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            is_ssbr_qualifying: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    let ssbr = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SSBR");
    assert!(ssbr.is_some(), "Expected SSBR to be awarded in 2023/24");
    assert_eq!(ssbr.unwrap().relief_name, "Supporting Small Business Relief");
    assert_eq!(seg.net_segment_payable, dec!(600.00));
}

#[tokio::test]
async fn test_england_supporting_small_business_relief_2026_cap_800() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2026, 3, 31).unwrap()),
                    rateable_value: dec!(10000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(16000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            is_ssbr_qualifying: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    let ssbr = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SSBR");
    assert!(ssbr.is_some(), "Expected SSBR to be awarded in 2026/27");
    assert_eq!(ssbr.unwrap().relief_name, "Supporting Small Business Relief");
    assert_eq!(seg.net_segment_payable, dec!(800.00));
}

#[test]
fn test_ssbr_compounds_into_year_2() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2017-04-01",
                    "effective_to": "2023-03-31",
                    "rateable_value": 10000.00
                },
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 16000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2023-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_sole_property": true,
            "is_ssbr_qualifying": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);

    assert_eq!(response.annual_billing_segments.len(), 2);

    // Year 1 (2023/24): Previous bill = £0 (100% SBRR on £10k RV). Capped to £600.
    let seg1 = &response.annual_billing_segments[0];
    assert_eq!(seg1.financial_year, "2023/24");
    assert_eq!(seg1.net_segment_payable, dec!(600.00));
    assert!(seg1.itemized_reliefs.iter().any(|r| r.relief_code == "SSBR"));

    // Year 2 (2024/25): Previous capped bill = £600. Capped to £600 + £600 = £1,200.
    let seg2 = &response.annual_billing_segments[1];
    assert_eq!(seg2.financial_year, "2024/25");
    assert_eq!(seg2.net_segment_payable, dec!(1200.00));
    assert!(seg2.itemized_reliefs.iter().any(|r| r.relief_code == "SSBR"));
}

#[test]
fn test_ssbr_not_awarded_for_new_property_without_baseline() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2024-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_ssbr_qualifying": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);

    assert_eq!(response.annual_billing_segments.len(), 1);
    let seg = &response.annual_billing_segments[0];

    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "SSBR"), "New property without 31 March 2023 bill must not receive SSBR");
    assert_eq!(seg.gross_liability, dec!(19960.00));
    assert_eq!(seg.net_segment_payable, dec!(19960.00));
}

#[test]
fn test_ssbr_not_awarded_to_charities() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CHARITY",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2017-04-01",
                    "effective_to": "2023-03-31",
                    "rateable_value": 10000.00
                },
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 16000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2023-04-01",
            "end_date": "2024-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_ssbr_qualifying": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);

    let seg = &response.annual_billing_segments[0];
    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "SSBR"), "Charity must not receive SSBR");
}

#[tokio::test]
async fn test_sector_relief_england_2025_vs_2026() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // High street shop (RV £30,000) occupied across 2025/26 and 2026/27 with RHL flag
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(30000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rhl_qualifying: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    assert_eq!(response.annual_billing_segments.len(), 2);

    // Segment 1 (2025/26): 40% RHL relief applies
    let seg1 = &response.annual_billing_segments[0];
    assert_eq!(seg1.financial_year, "2025/26");
    let rhl1 = seg1.itemized_reliefs.iter().find(|r| r.relief_code == "RHL_2025");
    assert!(rhl1.is_some(), "Expected RHL_2025 in 2025/26");
    assert_eq!(rhl1.unwrap().discount_percentage, dec!(40.00));

    // Segment 2 (2026/27): Temporary RHL percentage relief ceased on 31 March 2026 in England
    let seg2 = &response.annual_billing_segments[1];
    assert_eq!(seg2.financial_year, "2026/27");
    assert!(!seg2.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")),
        "Expected NO RHL relief in England 2026/27");
}

#[test]
fn test_england_multiplier_2025_26_standard_is_0555() {
    // Reproduction for Issue #48
    // LGFA 1988 Sched 7 para A4 & S.I. 2024/1321:
    // Small business multiplier is frozen at 49.9p (0.499).
    // Standard multiplier is uprated by CPI (1.7%) from 0.546 to 0.555 (55.5p).
    let engine = CalculationEngine::new(business_rates_engine::db::repository::ParameterRepository::new_seeded());
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2025-04-01",
            "end_date": "2026-03-31"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 60000.00
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let resp = engine.calculate(request);
    let seg = &resp.annual_billing_segments[0];
    assert_eq!(seg.multiplier_used.rate, dec!(0.555));
    assert_eq!(seg.multiplier_used.multiplier_type, "STANDARD");
    assert_eq!(seg.gross_liability, dec!(33300.00));
}

#[test]
fn test_england_small_business_multiplier_boundary_at_51000() {
    // Reproduction for Issue #49
    // S.I. 2017/102 art. 2: condition is rateable value "is not more than £50,999".
    // S.I. 2026/152 regs 3-5: small business multipliers apply where "A is less than £51,000",
    // standard multipliers apply where "A is £51,000 or more".
    let engine = CalculationEngine::new(business_rates_engine::db::repository::ParameterRepository::new_seeded());

    let calc = |start: &str, end: &str, rv: f64, rhl: bool| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "occupation_period": { "start_date": start, "end_date": end },
            "hereditament": {
                "jurisdiction": "ENGLAND",
                "scat_code": "091",
                "rateable_value_history": [
                    { "effective_from": "2023-04-01", "effective_to": null, "rateable_value": rv }
                ]
            },
            "occupancy_type": "OCCUPIED",
            "qualifying_relief_flags": {
                "apply_transitional_relief": false,
                "is_rhl_qualifying": rhl
            }
        });
        let req: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
        let resp = engine.calculate(req);
        (
            resp.annual_billing_segments[0].multiplier_used.rate,
            resp.annual_billing_segments[0].multiplier_used.multiplier_type.clone(),
        )
    };

    // 1. England 2024/25 at RV £51,000 pays standard multiplier 0.546 (not small business 0.499)
    let (rate_2024, type_2024) = calc("2024-04-01", "2025-03-31", 51000.0, false);
    assert_eq!(rate_2024, dec!(0.546));
    assert_eq!(type_2024, "STANDARD");

    // 2. England 2026/27 at RV £51,000 pays standard multiplier 0.480 (not small business 0.432)
    let (rate_2026, type_2026) = calc("2026-04-01", "2027-03-31", 51000.0, false);
    assert_eq!(rate_2026, dec!(0.480));
    assert_eq!(type_2026, "STANDARD");

    // 3. England 2026/27 RHL at RV £51,000 pays standard RHL multiplier 0.430 (not small business RHL 0.382)
    let (rate_2026_rhl, type_2026_rhl) = calc("2026-04-01", "2027-03-31", 51000.0, true);
    assert_eq!(rate_2026_rhl, dec!(0.430));
    assert_eq!(type_2026_rhl, "STANDARD_RHL");
}

#[test]
fn test_sbrr_taper_uses_statutory_e_divisor_england() {
    // Reproduction for Issue #50. Chargeable amount = (A x M) / E (LGFA 1988 Sched 4ZA para 10(6)), where for
    // RV £12,001-£15,000 E = 3000 / (A - 12000) calculated to three decimal places (S.I. 2017/102 art. 3(3), (5);
    // S.I. 2023/1250 reg 3(3)-(4)).
    let engine = CalculationEngine::new(ParameterRepository::new_seeded());
    let net = |rv: f64| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "hereditament": {
                "jurisdiction": "ENGLAND",
                "scat_code": "091",
                "rateable_value_history": [ { "effective_from": "2023-04-01", "effective_to": null, "rateable_value": rv } ]
            },
            "occupation_period": { "start_date": "2024-04-01", "end_date": "2025-03-31" },
            "occupancy_type": "OCCUPIED",
            "qualifying_relief_flags": { "apply_transitional_relief": false, "is_small_business_sole_property": true }
        });
        let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
        engine.calculate(request).annual_billing_segments[0].net_segment_payable
    };
    assert_eq!(net(14000.0), dec!(4657.33)); // E = 1.500: 6,986.00 / 1.5
    assert_eq!(net(12001.0), dec!(2.00));    // E = 3000.000: 5,988.50 / 3000
    assert_eq!(net(14999.0), dec!(7484.50)); // E = 3000/2999 = 1.000 to 3 dp: no relief
    assert_eq!(net(13500.0), dec!(3368.25)); // E = 2.000
    assert_eq!(net(12000.0), dec!(0.00));    // E = 5,000,000
}

#[test]
fn test_empty_rate_free_period_is_calendar_months() {
    // Reproduction for Issue #51. S.I. 2008/386 reg 4(a)-(b) and W.S.I. 2008/2499 reg 4(a)-(b): no empty rate while
    // unoccupied "for a continuous period not exceeding three months" (six months for qualifying industrial
    // hereditaments). "Month" means calendar month (Interpretation Act 1978 Sched 1).
    let engine = CalculationEngine::new(ParameterRepository::new_seeded());
    let net = |jur: &str, start: &str, end: &str, industrial: bool| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "hereditament": {
                "jurisdiction": jur,
                "scat_code": "091",
                "rateable_value_history": [ { "effective_from": "2017-04-01", "effective_to": null, "rateable_value": 30000.00 } ]
            },
            "occupation_period": { "start_date": start, "end_date": end },
            "occupancy_type": "UNOCCUPIED",
            "qualifying_relief_flags": {
                "apply_transitional_relief": false,
                "days_previously_unoccupied": 0,
                "is_industrial_hereditament": industrial
            }
        });
        let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
        engine.calculate(request).summary_financials.total_net_payable
    };
    // Empty from 1 Dec 2024: three months end 28 Feb 2025 (90 days); liable for 31 days in March.
    // 30,000 x 0.499 x 31 / 365 = £1,271.42
    assert_eq!(net("ENGLAND", "2024-12-01", "2025-03-31", false), dec!(1271.42));
    // Empty from 1 Apr 2024: three months end 30 Jun (91 days); liable 274 days: 30,000 x 0.499 x 274/365 = £11,237.75
    assert_eq!(net("ENGLAND", "2024-04-01", "2025-03-31", false), dec!(11237.75));
    // Industrial from 1 Apr 2024: six months end 30 Sep (183 days); liable 182 days = £7,464.49
    assert_eq!(net("ENGLAND", "2024-04-01", "2025-03-31", true), dec!(7464.49));
    // Wales, empty from 1 Nov 2024: three months end 31 Jan 2025 (92 days); liable 59 days: 30,000 x 0.562 x 59/365 = £2,725.32
    assert_eq!(net("WALES", "2024-11-01", "2025-03-31", false), dec!(2725.32));
}

#[test]
fn test_england_empty_property_historical_low_value_thresholds() {
    // Reproduction for Issue #52.
    // Statutory thresholds under S.I. 2008/386 reg 4(g) as amended:
    // - 2008/09: £2,200 (S.I. 2008/386 reg 4(g))
    // - 2009/10: £15,000 (S.I. 2008/3283 reg 2)
    // - 2010/11: £18,000 (S.I. 2010/408 reg 2(2)(b))
    // - 2011/12-2016/17: £2,600 (S.I. 2010/408 reg 2(2)(a))
    // - 2017/18 onwards: £2,900 (S.I. 2017/102 art 4)
    let engine = CalculationEngine::new(ParameterRepository::new_seeded());
    let is_exempt = |start: &str, end: &str, rv: f64| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "hereditament": {
                "jurisdiction": "ENGLAND",
                "scat_code": "091",
                "rateable_value_history": [ { "effective_from": "1990-04-01", "effective_to": None::<String>, "rateable_value": rv } ]
            },
            "occupation_period": { "start_date": start, "end_date": end },
            "occupancy_type": "UNOCCUPIED",
            "qualifying_relief_flags": {
                "apply_transitional_relief": false,
                "days_previously_unoccupied": 91
            }
        });
        let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
        let resp = engine.calculate(request);
        resp.annual_billing_segments[0].itemized_reliefs.iter().any(|r| r.relief_code == "EMPTY_LOW_VALUE")
    };

    // 1. 2008/09 (£2,200 threshold)
    assert!(is_exempt("2008-04-01", "2009-03-31", 2199.00), "RV £2,199 should be exempt in 2008/09");
    assert!(!is_exempt("2008-04-01", "2009-03-31", 2500.00), "RV £2,500 should NOT be exempt in 2008/09 (threshold £2,200)");

    // 2. 2009/10 (£15,000 threshold under S.I. 2008/3283)
    assert!(is_exempt("2009-04-01", "2010-03-31", 14999.00), "RV £14,999 should be exempt in 2009/10");
    assert!(!is_exempt("2009-04-01", "2010-03-31", 15000.00), "RV £15,000 should NOT be exempt in 2009/10");

    // 3. 2010/11 (£18,000 threshold under S.I. 2010/408)
    assert!(is_exempt("2010-04-01", "2011-03-31", 17999.00), "RV £17,999 should be exempt in 2010/11");
    assert!(!is_exempt("2010-04-01", "2011-03-31", 18000.00), "RV £18,000 should NOT be exempt in 2010/11");

    // 4. 2011/12-2016/17 (£2,600 threshold under S.I. 2010/408)
    assert!(is_exempt("2011-04-01", "2012-03-31", 2599.00), "RV £2,599 should be exempt in 2011/12");
    assert!(!is_exempt("2011-04-01", "2012-03-31", 2700.00), "RV £2,700 should NOT be exempt in 2011/12 (threshold £2,600)");

    // 5. 2017/18 onwards (£2,900 threshold under S.I. 2017/102)
    assert!(is_exempt("2017-04-01", "2018-03-31", 2899.00), "RV £2,899 should be exempt in 2017/18");
    assert!(!is_exempt("2017-04-01", "2018-03-31", 2900.00), "RV £2,900 should NOT be exempt in 2017/18");
}

#[test]
fn test_statutory_installments_whole_pounds_si_1989_1058() {
    // Reproduction for Issue #53.
    // Under S.I. 1989/1058 Schedule 1 Part I para 1(4)-(5):
    // All but the first instalment must be whole pounds (A = C / D rounded to nearest pound),
    // and the first instalment takes the balancing difference B = C - (D - 1) * A.
    let engine = CalculationEngine::new(ParameterRepository::new_seeded());
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 14000
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false,
            "is_small_business_sole_property": true
        }
    });
    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let resp = engine.calculate(request);
    let seg = &resp.annual_billing_segments[0];
    let installments = seg.statutory_installments.as_ref().expect("installments present");
    assert_eq!(installments.len(), 10);
    // Net payable: £4,657.33. C / 10 = £465.733 -> A = £466.00.
    // B = £4,657.33 - 9 * £466 = £463.33.
    assert_eq!(installments[0].amount, dec!(463.33));
    for inst in &installments[1..] {
        assert_eq!(inst.amount, dec!(466.00));
        assert_eq!(inst.amount.fract(), dec!(0.00), "all subsequent instalments must be whole pounds");
    }
}

#[test]
fn test_england_transitional_relief_greater_london_small_band_threshold_28000() {
    // Reproduction for Issue #54.
    // Under S.I. 2022/1403 reg 10(5), a hereditament in Greater London with RV <= £28,000
    // is a small hereditament (upward cap +5% in 2023/24).
    // Outside London, the small band threshold is <= £20,000.
    // For RV £15,000 (2022/23) rising to £25,000 (2023 list):
    // In Greater London, £25,000 is small (cap +5%):
    // Base liability = 15,000 x 0.499 = £7,485.00; capped = 7,485.00 x 1.05 = £7,859.25.
    // Outside London, £25,000 is medium (cap +15%):
    // Capped = 7,485.00 x 1.15 = £8,607.75.
    let engine = CalculationEngine::new(ParameterRepository::new_seeded());
    let calc = |is_london: bool| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "occupation_period": {
                "start_date": "2023-04-01",
                "end_date": "2024-03-31"
            },
            "hereditament": {
                "jurisdiction": "ENGLAND",
                "scat_code": "091",
                "rateable_value_history": [
                    {
                        "effective_from": "2017-04-01",
                        "effective_to": "2023-03-31",
                        "rateable_value": 15000
                    },
                    {
                        "effective_from": "2023-04-01",
                        "effective_to": null,
                        "rateable_value": 25000
                    }
                ]
            },
            "occupancy_type": "OCCUPIED",
            "qualifying_relief_flags": {
                "apply_transitional_relief": true,
                "is_greater_london": is_london
            }
        });
        let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
        let resp = engine.calculate(request);
        resp.summary_financials.total_net_payable
    };

    assert_eq!(calc(true), dec!(7859.25), "Greater London RV £25,000 qualifies as small (5% cap)");
    assert_eq!(calc(false), dec!(8607.75), "Outside London RV £25,000 is medium (15% cap)");
}

#[tokio::test]
async fn test_england_2026_transitional_supplement() {
    // Reproduction for Issue #55:
    // Under S.I. 2026/81 reg 16, for a defined hereditament (in the list on 31 March 2026 and each day since)
    // in England for 2026/27 where regulation 12 (transitional relief) does NOT apply,
    // and neither heat networks, public lavatories, rural, nor charitable relief applies,
    // multiplier M is replaced by (M + RS) in charging formulas, where RS = 0.010 (1p).
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2026, 3, 31).unwrap()),
                    rateable_value: dec!(40000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(40000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(true),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request.clone());
    let seg = &response.annual_billing_segments[0];

    // England small business multiplier for RV £40,000 is 0.432.
    // Base liability in 2025/26 was 40,000 * 0.499 = £19,960.
    // 2026/27 upward cap is 15% (medium band) -> capped liability is £22,954.
    // Since gross liability at 0.432 (£17,280) is below capped liability (£22,954),
    // regulation 12 does NOT apply (no transitional relief awarded).
    // Therefore, regulation 16 applies: M becomes (M + RS) = 0.432 + 0.010 = 0.442.
    // Gross and net liability must be £40,000 * 0.442 = £17,680.00.
    assert_eq!(seg.multiplier_used.rate, dec!(0.442));
    assert_eq!(seg.gross_liability, dec!(17680.00));
    assert_eq!(seg.net_segment_payable, dec!(17680.00));

    // 2. Charity exemption under S.I. 2026/81 reg 16(1)(c):
    // Paragraph 2 of Schedule 4ZB to the Act (charitable rate relief) excludes reg 16.
    // Multiplier remains M = 0.432 (no TRS 1p supplement).
    // Gross = 40,000 * 0.432 = £17,280; 80% mandatory relief = £13,824; net = £3,456.00.
    let mut charity_req = request.clone();
    charity_req.party_type = PartyType::CHARITY;
    let charity_resp = engine.calculate(charity_req);
    let charity_seg = &charity_resp.annual_billing_segments[0];
    assert_eq!(charity_seg.multiplier_used.rate, dec!(0.432));
    assert_eq!(charity_seg.gross_liability, dec!(17280.00));
    assert_eq!(charity_seg.net_segment_payable, dec!(3456.00));

    // 3. New property created on 1 April 2026 (not a defined hereditament under reg 5(2)):
    // Shown only from 1 April 2026, so not in list on 31 March 2026 -> no TRS 1p supplement.
    let mut new_req = request.clone();
    new_req.hereditament.rateable_value_history = vec![
        RvEntry {
            effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            effective_to: None,
            rateable_value: dec!(40000.00),
        },
    ];
    let new_resp = engine.calculate(new_req);
    let new_seg = &new_resp.annual_billing_segments[0];
    assert_eq!(new_seg.multiplier_used.rate, dec!(0.432));
    assert_eq!(new_seg.gross_liability, dec!(17280.00));
    assert_eq!(new_seg.net_segment_payable, dec!(17280.00));
}

#[tokio::test]
async fn test_england_small_business_multiplier_threshold_pre_2017() {
    let app = create_test_app();

    // 1. ENG-SMALLTHRESH-2015/16-30000: England 2015/16 at RV £30,000 -> STANDARD multiplier 49.3p (outside London)
    let payload_2015 = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2015-04-01",
            "end_date": "2016-03-31"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2005-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.0
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false
        }
    });
    let req1 = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&payload_2015).unwrap()))
        .unwrap();
    let res1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(res1.status(), StatusCode::OK);
    let body1: Value = serde_json::from_str(&body_to_string(res1.into_body()).await).unwrap();
    let seg1 = &body1["annual_billing_segments"][0];
    assert_eq!(seg1["multiplier_used"]["type"], "STANDARD", "RV £30,000 in 2015/16 must use STANDARD multiplier");
    assert_eq!(seg1["multiplier_used"]["rate"], 0.493, "Standard multiplier in 2015/16 is 0.493");

    // 2. ENG-SMALLTHRESH-2010/11-30000: England 2010/11 at RV £30,000 -> STANDARD multiplier 41.4p
    let payload_2010 = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2010-04-01",
            "end_date": "2011-03-31"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2005-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.0
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false
        }
    });
    let req2 = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&payload_2010).unwrap()))
        .unwrap();
    let res2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::OK);
    let body2: Value = serde_json::from_str(&body_to_string(res2.into_body()).await).unwrap();
    let seg2 = &body2["annual_billing_segments"][0];
    assert_eq!(seg2["multiplier_used"]["type"], "STANDARD", "RV £30,000 in 2010/11 must use STANDARD multiplier");
    assert_eq!(seg2["multiplier_used"]["rate"], 0.414, "Standard multiplier in 2010/11 is 0.414");

    // 3. Greater London comparison: RV £20,000 in 2015/16
    // Outside London (threshold £18,000): STANDARD multiplier 49.3p
    // Inside London (threshold £25,500 under S.I. 2012/148): SMALL_BUSINESS multiplier 48.0p
    let payload_london = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2015-04-01",
            "end_date": "2016-03-31"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2010-04-01",
                    "effective_to": null,
                    "rateable_value": 20000.0
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false,
            "is_greater_london": true
        }
    });
    let req_lon = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&payload_london).unwrap()))
        .unwrap();
    let res_lon = app.clone().oneshot(req_lon).await.unwrap();
    assert_eq!(res_lon.status(), StatusCode::OK);
    let body_lon: Value = serde_json::from_str(&body_to_string(res_lon.into_body()).await).unwrap();
    let seg_lon = &body_lon["annual_billing_segments"][0];
    assert_eq!(seg_lon["multiplier_used"]["type"], "SMALL_BUSINESS", "RV £20,000 in Greater London qualifies for SMALL_BUSINESS");
    assert_eq!(seg_lon["multiplier_used"]["rate"], 0.480);
}


