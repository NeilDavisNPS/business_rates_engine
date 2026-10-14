//! Suite G: Scottish Property Rates, SBBS Tapers, 40% Hospitality & Transitional Relief
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
async fn test_scotland_2026_multipliers_basic_intermediate_higher() {
    let repo = ParameterRepository::new_seeded();

    // Reproduction for Issue #18 (Part 1): SSI 2026/39 & SSI 2026/68 regs 5-6
    // 1. Basic (RV <= £51,000): 0.481
    let m_basic = repo.resolve_multiplier(Jurisdiction::SCOTLAND, "2026/27", dec!(30000.00)).unwrap();
    assert_eq!(m_basic.rate, dec!(0.481));
    assert_eq!(m_basic.multiplier_type, "BASIC");

    // 2. Intermediate (£51,001 <= RV <= £100,000): 0.535 (0.481 + 0.054)
    let m_inter = repo.resolve_multiplier(Jurisdiction::SCOTLAND, "2026/27", dec!(75000.00)).unwrap();
    assert_eq!(m_inter.rate, dec!(0.535));
    assert_eq!(m_inter.multiplier_type, "INTERMEDIATE");

    // 3. Higher (RV > £100,000): 0.548 (0.481 + 0.067)
    let m_higher = repo.resolve_multiplier(Jurisdiction::SCOTLAND, "2026/27", dec!(150000.00)).unwrap();
    assert_eq!(m_higher.rate, dec!(0.548));
    assert_eq!(m_higher.multiplier_type, "HIGHER");
}

#[tokio::test]
async fn test_scotland_sbbs_taper_band_12k_to_15k() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #19: Scotland 2024/25, sole property, RV £13,500
    // Under SSI 2024/4 reg 3, relief tapers from 100% to 25%:
    // Relief = 25% + 75% * (15,000 - 13,500)/3,000 = 62.5%
    // Gross = 13,500 * 0.498 = £6,723.00
    // Relief amount = £4,201.88 -> Net = £2,521.13 (currently fails with 25% / £5,042.25)
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 13500.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_sole_property": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    let sbbs = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SBBS").expect("SBBS should apply");
    assert_eq!(sbbs.discount_percentage, dec!(62.50));
    assert_eq!(seg.net_segment_payable, dec!(2521.12));
}

#[tokio::test]
async fn test_scotland_sbbs_taper_band_15k_to_20k() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #19: Scotland 2024/25, sole property, RV £17,000
    // Under SSI 2024/4 reg 3, relief tapers from 25% to 0%:
    // Relief = 25% * (20,000 - 17,000)/5,000 = 15.0%
    // Gross = 17,000 * 0.498 = £8,466.00
    // Relief amount = £1,269.90 -> Net = £7,196.10 (currently fails with 0% / £8,466.00)
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 17000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_sole_property": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    let sbbs = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SBBS").expect("SBBS should apply");
    assert_eq!(sbbs.discount_percentage, dec!(15.00));
    assert_eq!(seg.net_segment_payable, dec!(7196.10));
}

#[tokio::test]
async fn test_scotland_sbbs_multiple_properties_rule_reg_4() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #19: Multiple properties in Scotland (SSI 2024/4 reg 4)
    // Ratepayer has multiple entries with cumulative RV £25,000 (<= £35,000 cap).
    // This individual property has RV £10,000. Under Reg 4, it receives 25% relief.
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 10000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_multiple_properties": true,
            "small_business_cumulative_rv": 25000.00,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    let sbbs = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SBBS").expect("Multiple-property SBBS should apply under reg 4");
    assert_eq!(sbbs.discount_percentage, dec!(25.00));
    assert!(sbbs.statutory_authority.contains("reg. 4"));
}

#[tokio::test]
async fn test_scotland_sbbs_multiple_properties_exceeding_35k_cap() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #19: Multiple properties in Scotland exceeding £35k cumulative cap
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 10000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_multiple_properties": true,
            "small_business_cumulative_rv": 40000.00,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "SBBS"),
        "Multiple properties exceeding £35,000 cumulative RV must receive 0% SBBS relief");
}

#[tokio::test]
async fn test_scotland_2026_rhl_retail_15_percent_shop() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #18: Scotland 2026/27, mainland retail shop (SCAT 225), RV £30,000
    // Under SSI 2026/39, basic multiplier is 0.481 -> Gross = £30,000 * 0.481 = £14,430.00
    // Under SSI 2026/69 reg. 4, retail receives 15% relief
    // RHL = 15% of £14,430.00 = £2,164.50
    // Net = £12,265.50
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "225",
            "rateable_value_history": [
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2026-04-01",
            "end_date": "2027-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_rhl_qualifying": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.multiplier_used.rate, dec!(0.481));
    assert_eq!(seg.gross_liability, dec!(14430.00));
    let rhl = seg.itemized_reliefs.iter().find(|r| r.relief_code.starts_with("RHL")).expect("RHL should apply");
    assert_eq!(rhl.discount_percentage, dec!(15.00));
    assert_eq!(rhl.amount_deducted, dec!(2164.50));
    assert_eq!(seg.net_segment_payable, dec!(12265.50));
}

#[tokio::test]
async fn test_scotland_2026_rhl_hospitality_40_percent_pub() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #18 (Part 2 & Part 3):
    // Scotland 2026/27, mainland pub (SCAT 209), RV £30,000, is_rhl_qualifying
    // Under SSI 2026/39, basic multiplier is 0.481 -> Gross = £30,000 * 0.481 = £14,430.00
    // Under SSI 2026/69 reg. 4A (added by SSI 2026/135), hospitality receives 40% relief
    // RHL = 40% of £14,430.00 = £5,772.00
    // Net = £8,658.00
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "209",
            "rateable_value_history": [
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2026-04-01",
            "end_date": "2027-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_rhl_qualifying": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.multiplier_used.rate, dec!(0.481));
    assert_eq!(seg.gross_liability, dec!(14430.00));
    let rhl = seg.itemized_reliefs.iter().find(|r| r.relief_code.starts_with("RHL")).expect("RHL should apply");
    assert_eq!(rhl.discount_percentage, dec!(40.00));
    assert_eq!(rhl.amount_deducted, dec!(5772.00));
    assert_eq!(seg.net_segment_payable, dec!(8658.00));
}

#[tokio::test]
async fn test_scotland_2026_remote_mainland_rhl_100_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // SSI 2026/69 reg 5: Cape Wrath, Knoydart, Scoraig receive 100% relief (same as islands)
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "225",
            "rateable_value_history": [
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2026-04-01",
            "end_date": "2027-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_rhl_qualifying": true,
            "is_scottish_remote_mainland": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    let rhl = seg.itemized_reliefs.iter().find(|r| r.relief_code == "RHL_SCOTLAND_ISLANDS_2026").expect("Remote mainland receives 100% relief");
    assert_eq!(rhl.discount_percentage, dec!(100.00));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_sector_relief_scotland_mainland_vs_island_2026() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // Case A: Mainland Scotland retail shop RV £50,000 in 2026/27 -> 15% relief
    let req_mainland = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(50000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rhl_qualifying: true,
            is_island_property: false,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_mainland = engine.calculate(req_mainland);
    let seg_m = &res_mainland.annual_billing_segments[0];
    let rhl_m = seg_m.itemized_reliefs.iter().find(|r| r.relief_code == "RHL_SCOTLAND_MAINLAND_2026");
    assert!(rhl_m.is_some(), "Expected Mainland Scotland RHL relief in 2026/27");
    assert_eq!(rhl_m.unwrap().discount_percentage, dec!(15.00));
    assert!(rhl_m.unwrap().relief_name.contains("Scottish"));

    // Case B: Island property in Scotland (Islands (Scotland) Act 2018) RV £50,000 -> 100% relief
    let req_island = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "209".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(50000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rhl_qualifying: true,
            is_island_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_island = engine.calculate(req_island);
    let seg_i = &res_island.annual_billing_segments[0];
    let rhl_i = seg_i.itemized_reliefs.iter().find(|r| r.relief_code == "RHL_SCOTLAND_ISLANDS_2026");
    assert!(rhl_i.is_some(), "Expected Scottish Islands RHL relief in 2026/27");
    assert_eq!(rhl_i.unwrap().discount_percentage, dec!(100.00));
    assert_eq!(seg_i.net_segment_payable, dec!(0.00));
    assert!(rhl_i.unwrap().statutory_authority.contains("Islands (Scotland) Act 2018"));

    // Case C: Mainland property exceeding £100,000 RV does NOT qualify for 15% relief
    let req_mainland_high = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "209".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(150000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rhl_qualifying: true,
            is_island_property: false,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_mainland_high = engine.calculate(req_mainland_high);
    let seg_mh = &res_mainland_high.annual_billing_segments[0];
    assert!(!seg_mh.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")),
        "Mainland property over £100k RV must not receive RHL relief");

    // Case D: Mainland Scotland hospitality premises (pub SCAT 209) RV £50,000 in 2026/27 -> 40% relief (SSI 2026/135)
    let req_hosp = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "209".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(50000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rhl_qualifying: true,
            is_hospitality_premises: true,
            is_island_property: false,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_hosp = engine.calculate(req_hosp);
    let seg_h = &res_hosp.annual_billing_segments[0];
    let rhl_h = seg_h.itemized_reliefs.iter().find(|r| r.relief_code == "RHL_SCOTLAND_HOSPITALITY_2026");
    assert!(rhl_h.is_some(), "Expected Scottish Hospitality RHL relief in 2026/27");
    assert_eq!(rhl_h.unwrap().discount_percentage, dec!(40.00));
}

#[tokio::test]
async fn test_scotland_2026_self_catering_licensing_rhl_qualification() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: Unlicensed self-catering (SCAT 249) -> Excluded from RHL under SSI 2026/69 reg. 4(2)(b)
    let json_unlicensed = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2026-04-01",
            "end_date": "2027-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_rhl_qualifying": true,
            "has_short_term_let_licence": false,
            "apply_transitional_relief": false,
            "holiday_let_actual_days_let": 200,
            "holiday_let_days_available": 200
        }
    });

    let req_unlicensed: LiabilityCalculationRequest = serde_json::from_value(json_unlicensed).unwrap();
    let res_unlicensed = engine.calculate(req_unlicensed);
    let seg_un = &res_unlicensed.annual_billing_segments[0];
    assert!(!seg_un.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")),
        "Unlicensed self-catering holiday accommodation must be excluded from Scottish RHL relief");

    // Case 2: Licensed self-catering (SCAT 249) -> Receives 15% retail/leisure RHL under SSI 2026/69 reg. 4(1)
    let json_licensed = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2026-04-01",
            "end_date": "2027-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_rhl_qualifying": true,
            "has_short_term_let_licence": true,
            "apply_transitional_relief": false,
            "holiday_let_actual_days_let": 200,
            "holiday_let_days_available": 200
        }
    });

    let req_licensed: LiabilityCalculationRequest = serde_json::from_value(json_licensed).unwrap();
    let res_licensed = engine.calculate(req_licensed);
    let seg_lic = &res_licensed.annual_billing_segments[0];
    let rhl = seg_lic.itemized_reliefs.iter().find(|r| r.relief_code.starts_with("RHL")).expect("Licensed SCAT 249 should receive RHL");
    assert_eq!(rhl.discount_percentage, dec!(15.00));
    assert_eq!(rhl.amount_deducted, dec!(2164.50));
}

#[tokio::test]
async fn test_scotland_2026_transitional_caps() {
    let repo = ParameterRepository::new_seeded();

    // Reproduction for Issue #18 (Part 4): SSI 2026/70 reg 9 & SSI 2026/135 reg 4
    // Small (<= £20,000): X = 1.150 (+15%)
    let cap_sm = repo.resolve_transitional_cap(Jurisdiction::SCOTLAND, "2026/27", dec!(15000.00)).unwrap();
    assert_eq!(cap_sm.upward_cap_percentage, dec!(15.00));

    // Medium (<= £100,000): X = 1.300 (+30%)
    let cap_med = repo.resolve_transitional_cap(Jurisdiction::SCOTLAND, "2026/27", dec!(60000.00)).unwrap();
    assert_eq!(cap_med.upward_cap_percentage, dec!(30.00));

    // Large (> £100,000): X = 1.500 (+50%)
    let cap_lg = repo.resolve_transitional_cap(Jurisdiction::SCOTLAND, "2026/27", dec!(150000.00)).unwrap();
    assert_eq!(cap_lg.upward_cap_percentage, dec!(50.00));
}

#[tokio::test]
async fn test_scotland_2026_self_catering_licensed_transitional_cap_15_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // SSI 2026/135 reg 4: Licensed self-catering holiday accommodation (SCAT 249) receives 15% cap (X = 1.150)
    // regardless of rateable value (even if RV > £100k which normally has 50% cap).
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": "2026-03-31",
                    "rateable_value": 80000.00
                },
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 120000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2026-04-01",
            "end_date": "2027-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": true,
            "has_short_term_let_licence": true,
            "holiday_let_actual_days_let": 200,
            "holiday_let_days_available": 200
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    // 2025/26 base: £80,000 * 0.554 = £44,320.00 (S.S.I. 2025/38 reg. 5(4))
    // 2026/27 capped gross with X = 1.150: £44,320 * 1.150 = £50,968.00
    // 2026/27 gross: £120,000 * 0.548 (higher multiplier) = £65,760.00
    // TR adjustment: £65,760.00 - £50,968.00 = £14,792.00
    assert_eq!(seg.multiplier_used.rate, dec!(0.548));
    assert_eq!(seg.gross_liability, dec!(65760.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(14792.00));
    assert_eq!(seg.net_segment_payable, dec!(50968.00));
}

#[tokio::test]
async fn test_scottish_general_transitional_relief_2023_list() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Scottish property RV £10,000 in 2022/23 increasing to £18,000 in 2023/24 (Small property <= £20k)
    // 2022/23 base: £10,000 * 0.498 = £4,980.00
    // 2023/24 cap: +12.5% -> £4,980 * 1.125 = £5,602.50
    // 2023/24 gross: £18,000 * 0.498 = £8,964.00
    // Transitional Relief = £8,964.00 - £5,602.50 = £3,361.50
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
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
                    rateable_value: dec!(18000.00),
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

    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(3361.50));
    assert_eq!(response.summary_financials.total_transitional_adjustments, dec!(3361.50));
    assert_eq!(seg.net_segment_payable, dec!(5602.50));
}

#[tokio::test]
async fn test_scottish_sbbs_tr_2023_list_600_cap() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Scottish property with RV £14,000 in 2022/23 receiving 100% SBBS (net = £0)
    // In 2023/24, RV stays at £14,000. Under 2023 SBBS rules, £14k receives 25% taper.
    // Gross = £14,000 * 0.498 = £6,972.00
    // SBBS = 25% of £6,972 = £1,743.00
    // Net before SBBS TR = £5,229.00
    // Under SSI 2023/31, net bill increase over 2022/23 (£0) is capped to £600
    // SBBS TR deduction = £5,229.00 - £600.00 = £4,629.00
    // Net payable = £600.00
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2020, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2023, 3, 31).unwrap()),
                    rateable_value: dec!(14000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(14000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            is_sbbs_tr_qualifying: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    // Under SSI 2023/30 reg 3, RV £14,000 qualifies for 50% SBBS (£3,486.00).
    // Chargeable after SBBS = £3,486.00.
    // SBBS TR under SSI 2023/45 caps the net bill increase over previous year (£0) to £600.
    // SBBS TR deduction = £3,486.00 - £600.00 = £2,886.00.
    // Net segment payable = £600.00.
    let sbbs_tr = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SBBS_TR");
    assert!(sbbs_tr.is_some(), "Expected SBBS TR to be awarded");
    assert_eq!(sbbs_tr.unwrap().amount_deducted, dec!(2886.00));
    assert_eq!(sbbs_tr.unwrap().relief_name, "Small Business Bonus Scheme Transitional Relief");
    assert_eq!(seg.net_segment_payable, dec!(600.00));
}

#[tokio::test]
async fn test_scottish_sbbs_tr_2026_list_25_percent_cap() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Scottish property with RV £10,000 in 2025/26 receiving 100% SBBS (net = £0).
    // In 2026/27, RV revalued to £16,000 (basic rate 0.481 under SSI 2026/39).
    // Base liability = £10,000 * 0.498 = £4,980.00.
    // 2026 General TR (+15.0% under SSI 2026/70 reg 9): capped gross = £4,980 * 1.150 = £5,727.00.
    // Gross = £16,000 * 0.481 = £7,696.00 -> General TR deduction = £1,969.00.
    // Chargeable after General TR = £5,727.00.
    // Under SSI 2023/30 reg 3(3) / SSI 2026/68 reg 3(3), RV £16,000 receives 20% SBBS:
    // SBBS deduction = 20% of £5,727.00 = £1,145.40.
    // Net before SBBS TR = £5,727.00 - £1,145.40 = £4,581.60.
    // Under 2026 SBBS TR (SSI 2026/70 reg. 14), the net bill increase over previous year (£0) is capped to 25%:
    // Ratepayer pays 25% of £4,581.60 = £1,145.40.
    // SBBS TR deducts 75% of £4,581.60 = £3,436.20.
    // Net segment payable = £1,145.40.
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
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
            is_sbbs_tr_qualifying: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(1969.00));

    let sbbs_tr = seg.itemized_reliefs.iter().find(|r| r.relief_code == "SBBS_TR");
    assert!(sbbs_tr.is_some(), "Expected SBBS TR to be awarded in 2026/27");
    assert_eq!(sbbs_tr.unwrap().amount_deducted, dec!(3436.20));
    assert_eq!(seg.net_segment_payable, dec!(1145.40));
}

#[tokio::test]
async fn test_scottish_unoccupied_property_relief_50_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: Low-value threshold in Scotland is £1,700 under SSI 2018/77 (vs £2,900 in England)
    // RV £1,500 (< £1,700) -> 100% low value empty relief
    let low_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(1500.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };
    let low_res = engine.calculate(low_req);
    let low_seg = &low_res.annual_billing_segments[0];
    assert_eq!(low_seg.net_segment_payable, dec!(0.00));
    assert!(low_seg.itemized_reliefs.iter().any(|r| r.relief_code == "EMPTY_LOW_VALUE"));

    // Case 2: RV £10,000 (>= £1,700), commercial property, unoccupied for full 365 days in 2024/25.
    // Basic multiplier = 0.498. Gross = 10,000 * 0.498 = £4,980.00.
    // Under Scottish SSI 2018/77 reg 3, the first 91 days receive 50% relief (not 100% like England).
    // Relief = 4,980.00 * (91 / 365) * 0.50 = £620.65.
    // Net = £4,980.00 - £620.65 = £4,359.35.
    let unocc_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(10000.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };
    let unocc_res = engine.calculate(unocc_req);
    let unocc_seg = &unocc_res.annual_billing_segments[0];
    assert_eq!(unocc_seg.gross_liability, dec!(4980.00));
    let empty_rel = unocc_seg.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION").unwrap();
    assert_eq!(empty_rel.amount_deducted, dec!(620.79));
    assert_eq!(unocc_seg.net_segment_payable, dec!(4359.21));
}

#[tokio::test]
async fn test_scottish_renewable_energy_generation_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: RV £100,000 (<= £145,000) -> 100% relief under SSI 2010/336
    // 2024/25 Intermediate Multiplier 0.545. Gross = 100,000 * 0.545 = £54,500.00. Relief = £54,500.00. Net = £0.00.
    let req1 = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(100000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_renewable_energy_generation: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let res1 = engine.calculate(req1);
    let seg1 = &res1.annual_billing_segments[0];
    assert_eq!(seg1.gross_liability, dec!(54500.00));
    let rel1 = seg1.itemized_reliefs.iter().find(|r| r.relief_code == "RENEWABLE_ENERGY_RELIEF").unwrap();
    assert_eq!(rel1.discount_percentage, dec!(100.00));
    assert_eq!(rel1.amount_deducted, dec!(54500.00));
    assert_eq!(seg1.net_segment_payable, dec!(0.00));

    // Case 2: RV £180,000 (£145,000 < RV <= £200,000) -> 50% relief under SSI 2010/336
    // 2024/25 Higher Multiplier 0.559. Gross = 180,000 * 0.559 = £100,620.00. Relief 50% = £50,310.00. Net = £50,310.00.
    let req2 = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(180000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_renewable_energy_generation: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let res2 = engine.calculate(req2);
    let seg2 = &res2.annual_billing_segments[0];
    assert_eq!(seg2.gross_liability, dec!(100620.00));
    let rel2 = seg2.itemized_reliefs.iter().find(|r| r.relief_code == "RENEWABLE_ENERGY_RELIEF").unwrap();
    assert_eq!(rel2.discount_percentage, dec!(50.00));
    assert_eq!(rel2.amount_deducted, dec!(50310.00));
    assert_eq!(seg2.net_segment_payable, dec!(50310.00));
}

#[test]
fn test_scotland_2025_26_intermediate_and_higher_multipliers() {
    // Reproduction for Issue #56
    // S.S.I. 2025/37: basic property rate is 49.8p (0.498).
    // S.S.I. 2025/38 regs 5(4) and 6(4): additional factors 0.056 and 0.070,
    // giving intermediate rate 0.554 (55.4p) and higher rate 0.568 (56.8p).
    let engine = CalculationEngine::new(business_rates_engine::db::repository::ParameterRepository::new_seeded());

    let calc = |rv: f64| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "occupation_period": { "start_date": "2025-04-01", "end_date": "2026-03-31" },
            "hereditament": {
                "jurisdiction": "SCOTLAND",
                "scat_code": "091",
                "rateable_value_history": [
                    { "effective_from": "2023-04-01", "effective_to": null, "rateable_value": rv }
                ]
            },
            "occupancy_type": "OCCUPIED",
            "qualifying_relief_flags": {
                "apply_transitional_relief": false
            }
        });
        let req: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
        let resp = engine.calculate(req);
        (
            resp.annual_billing_segments[0].multiplier_used.rate,
            resp.annual_billing_segments[0].multiplier_used.multiplier_type.clone(),
            resp.annual_billing_segments[0].gross_liability,
        )
    };

    // 1. RV £60,000 pays Intermediate Property Rate 0.554 (not 0.545)
    let (rate_60k, type_60k, gross_60k) = calc(60000.0);
    assert_eq!(rate_60k, dec!(0.554));
    assert_eq!(type_60k, "INTERMEDIATE");
    assert_eq!(gross_60k, dec!(33240.00));

    // 2. RV £150,000 pays Higher Property Rate 0.568 (not 0.559)
    let (rate_150k, type_150k, gross_150k) = calc(150000.0);
    assert_eq!(rate_150k, dec!(0.568));
    assert_eq!(type_150k, "HIGHER");
    assert_eq!(gross_150k, dec!(85200.00));
}

#[test]
fn test_scotland_empty_property_2018_rules_90_percent_after_initial_period() {
    // Reproduction for Issue #58. S.S.I. 2018/77 reg 2: other unoccupied property pays 50% until it has been
    // unoccupied for a continuous period of 3 months, then 90% (reg 2(3)); industrial property pays nothing for
    // 6 months, then 90%. These rules applied from 1 April 2018 to 31 March 2023.
    let engine = CalculationEngine::new(ParameterRepository::new_seeded());
    let net = |start: &str, end: &str, industrial: bool| {
        let json = serde_json::json!({
            "party_type": "CORPORATE",
            "hereditament": {
                "jurisdiction": "SCOTLAND",
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
    // 2019/20 (366 days), RV £30,000 x 0.490 = £14,700: 50% for 91 days, then 90% for 275 days
    // 14,700 x (91/366 x 0.5 + 275/366 x 0.9) = £11,768.03
    assert_eq!(net("2019-04-01", "2020-03-31", false), dec!(11768.03));
    // Industrial, empty from 1 Sep 2019: nothing for six months (to 29 Feb 2020, 182 days), then 90% for the 31 days of
    // March 2020: 14,700 x 31/366 x 0.9 = £1,120.57
    assert_eq!(net("2019-09-01", "2020-03-31", true), dec!(1120.57));
}

#[test]
fn test_scotland_sbbs_2023_onward_published_thresholds_have_both_tapers() {
    // Reproduction for Issue #67:
    // Non-Domestic Rates (Levying) (Scotland) Regulations (S.S.I. 2023/31 & S.S.I. 2026/68 reg 3(3)):
    // SBBS from 2023/24 onwards has:
    // 1. Full 100% relief up to £12,000
    // 2. Linear taper 100% -> 25% for £12,001 to £15,000 (is_tapered: true)
    // 3. Linear taper 25% -> 0% for £15,001 to £20,000 (is_tapered: true)
    let repo = ParameterRepository::new_seeded();
    let sbrr_thresholds = repo.all_sbrr_thresholds();

    for fy in ["2023/24", "2024/25", "2025/26", "2026/27"] {
        let tap1 = sbrr_thresholds
            .iter()
            .find(|t| t.parameter_id == format!("sbbs_sco_tap_{}", fy))
            .unwrap_or_else(|| panic!("Missing sbbs_sco_tap_{}", fy));

        assert!(
            tap1.is_tapered,
            "sbbs_sco_tap_{} must be tapered (is_tapered = true) for £12k-£15k band",
            fy
        );
        assert_eq!(
            tap1.relief_percentage,
            dec!(100.00),
            "sbbs_sco_tap_{} starting relief percentage must be 100% tapering to 25%",
            fy
        );

        let tap2 = sbrr_thresholds
            .iter()
            .find(|t| t.parameter_id == format!("sbbs_sco_tap2_{}", fy))
            .unwrap_or_else(|| panic!("Missing sbbs_sco_tap2_{}", fy));

        assert!(
            tap2.is_tapered,
            "sbbs_sco_tap2_{} must be tapered (is_tapered = true) for £15k-£20k band",
            fy
        );
        assert_eq!(
            tap2.relief_percentage,
            dec!(25.00),
            "sbbs_sco_tap2_{} starting relief percentage must be 25% tapering to 0%",
            fy
        );
    }
}

#[tokio::test]
async fn test_scotland_large_business_supplement_and_property_rates_historical() {
    let app = create_test_app();

    // 1. SCO-LBS-2018/19-200000: Scotland 2018/19 at RV £200,000 -> 48.0p + 2.6p supplement = 50.6p (HIGHER)
    let payload_2018 = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2018-04-01",
            "end_date": "2019-03-31"
        },
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2005-04-01",
                    "effective_to": null,
                    "rateable_value": 200000.0
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
        .body(Body::from(serde_json::to_vec(&payload_2018).unwrap()))
        .unwrap();
    let res1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(res1.status(), StatusCode::OK);
    let body1: Value = serde_json::from_str(&body_to_string(res1.into_body()).await).unwrap();
    let seg1 = &body1["annual_billing_segments"][0];
    assert_eq!(seg1["multiplier_used"]["rate"], 0.506, "2018/19 RV £200,000 must pay 50.6p (48.0p + 2.6p LBS)");
    assert_eq!(seg1["multiplier_used"]["type"], "HIGHER");
    assert_eq!(seg1["gross_liability"], 101200.0);

    // 2. SCO-LBS-2014/15-40000: Scotland 2014/15 at RV £40,000 (above £35k) -> 47.1p + 1.1p = 48.2p (HIGHER)
    let payload_2014 = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2014-04-01",
            "end_date": "2015-03-31"
        },
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2005-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.0
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
        .body(Body::from(serde_json::to_vec(&payload_2014).unwrap()))
        .unwrap();
    let res2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::OK);
    let body2: Value = serde_json::from_str(&body_to_string(res2.into_body()).await).unwrap();
    let seg2 = &body2["annual_billing_segments"][0];
    assert_eq!(seg2["multiplier_used"]["rate"], 0.482, "2014/15 RV £40,000 must pay 48.2p (47.1p + 1.1p LBS)");
    assert_eq!(seg2["multiplier_used"]["type"], "HIGHER");

    // 3. SCO-IPR-2021/22-80000: Scotland 2021/22 at RV £80,000 -> Intermediate Property Rate 50.3p (INTERMEDIATE)
    let payload_2021 = serde_json::json!({
        "party_type": "CORPORATE",
        "occupation_period": {
            "start_date": "2021-04-01",
            "end_date": "2022-03-31"
        },
        "hereditament": {
            "jurisdiction": "SCOTLAND",
            "scat_code": "091",
            "rateable_value_history": [
                {
                    "effective_from": "2005-04-01",
                    "effective_to": null,
                    "rateable_value": 80000.0
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false
        }
    });
    let req3 = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&payload_2021).unwrap()))
        .unwrap();
    let res3 = app.clone().oneshot(req3).await.unwrap();
    assert_eq!(res3.status(), StatusCode::OK);
    let body3: Value = serde_json::from_str(&body_to_string(res3.into_body()).await).unwrap();
    let seg3 = &body3["annual_billing_segments"][0];
    assert_eq!(seg3["multiplier_used"]["rate"], 0.503, "2021/22 RV £80,000 must pay Intermediate Property Rate 50.3p");
    assert_eq!(seg3["multiplier_used"]["type"], "INTERMEDIATE");
}



