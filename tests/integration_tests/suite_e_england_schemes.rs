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
    let m1 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(30000.00), true, true, false).unwrap();
    assert_eq!(m1.rate, dec!(0.382));
    assert_eq!(m1.multiplier_type, "SMALL_BUSINESS_RHL");

    // 2. Small Business non-RHL (< £51k without RHL)
    let m2 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(30000.00), false, false, false).unwrap();
    assert_eq!(m2.rate, dec!(0.432));
    assert_eq!(m2.multiplier_type, "SMALL_BUSINESS");

    // 3. Standard RHL (£51k <= RV < £500k with RHL)
    let m3 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(100000.00), true, true, false).unwrap();
    assert_eq!(m3.rate, dec!(0.430));
    assert_eq!(m3.multiplier_type, "STANDARD_RHL");

    // 4. Standard non-RHL (£51k <= RV < £500k without RHL)
    let m4 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(100000.00), false, false, false).unwrap();
    assert_eq!(m4.rate, dec!(0.480));
    assert_eq!(m4.multiplier_type, "STANDARD");

    // 5. High-Value (RV >= £500k)
    let m5 = repo.resolve_multiplier_ex(Jurisdiction::ENGLAND, "2026/27", dec!(600000.00), false, false, false).unwrap();
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
