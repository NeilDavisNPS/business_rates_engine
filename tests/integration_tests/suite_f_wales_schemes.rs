//! Suite F: Welsh Multipliers, SBRR, Exemptions, Anti-Avoidance & Transitional Schemes
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
async fn test_wales_multiplier_2025_26_is_0568() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Wales 2025/26: RV £50,000 * 0.568 = £28,400.00 (not 0.562 / £28,100.00)
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 50000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2025-04-01",
            "end_date": "2026-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.multiplier_used.rate, dec!(0.568), "2025/26 Welsh multiplier must be 0.568 (+1% over 2024/25)");
    assert_eq!(seg.gross_liability, dec!(28400.00));
}

#[tokio::test]
async fn test_wales_2026_three_multipliers() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: Small Retail Hereditament (RV < £51,000, is_retail_premises = true) -> Retail Multiplier 0.350
    let retail_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "249".to_string(), // Shop
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(30000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_retail_premises: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let retail_res = engine.calculate(retail_req);
    let retail_seg = &retail_res.annual_billing_segments[0];
    assert_eq!(retail_seg.multiplier_used.rate, dec!(0.350));
    assert_eq!(retail_seg.multiplier_used.multiplier_type, "RETAIL");
    assert_eq!(retail_seg.gross_liability, dec!(10500.00));

    // Case 2: Standard Hereditament (RV <= £100,000, non-retail) -> Standard Multiplier 0.502
    let standard_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "096".to_string(), // Industrial/Office
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(30000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_retail_premises: false,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let standard_res = engine.calculate(standard_req);
    let standard_seg = &standard_res.annual_billing_segments[0];
    assert_eq!(standard_seg.multiplier_used.rate, dec!(0.502));
    assert_eq!(standard_seg.multiplier_used.multiplier_type, "STANDARD");
    assert_eq!(standard_seg.gross_liability, dec!(15060.00));

    // Case 3: Higher Hereditament (RV > £100,000) -> Higher Multiplier 0.515
    let higher_req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "249".to_string(),
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
            is_retail_premises: true, // Even if retail, RV > 100k takes Higher Multiplier
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };
    let higher_res = engine.calculate(higher_req);
    let higher_seg = &higher_res.annual_billing_segments[0];
    assert_eq!(higher_seg.multiplier_used.rate, dec!(0.515));
    assert_eq!(higher_seg.multiplier_used.multiplier_type, "HIGHER");
    assert_eq!(higher_seg.gross_liability, dec!(77250.00));
}

#[tokio::test]
async fn test_wales_retail_multiplier_given_to_retail_shop() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Wales 2026/27: Retail shop with RV £40,000 and is_retail_premises: true
    // Receives Welsh retail multiplier (0.350) under LGF(W)A 2024
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "225",
            "rateable_value_history": [
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2026-04-01",
            "end_date": "2027-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_retail_premises": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.multiplier_used.multiplier_type, "RETAIL");
    assert_eq!(seg.multiplier_used.rate, dec!(0.350));
    assert_eq!(seg.gross_liability, dec!(14000.00));
}

#[tokio::test]
async fn test_wales_retail_multiplier_not_given_to_holiday_let_or_pub_with_rhl_flag() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Wales 2026/27: Holiday let (SCAT 249) or pub with RV £40,000 flagged is_rhl_qualifying: true
    // Welsh 2026/27 retail multiplier (0.350) is restricted to small retail shops (< £51k),
    // and must NOT be awarded to holiday accommodation, pubs, restaurants, cafes or gyms.
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
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
            "is_retail_premises": false,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert_eq!(seg.multiplier_used.multiplier_type, "STANDARD");
    assert_eq!(seg.multiplier_used.rate, dec!(0.502), "Holiday let must receive standard multiplier 0.502, not retail 0.350");
    assert_eq!(seg.gross_liability, dec!(20080.00));
}

#[tokio::test]
async fn test_wales_higher_multiplier_exception_pays_standard_rate() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Wales 2026/27: Hereditament with RV £150,000 (> £100,000).
    // Normally pays HIGHER multiplier (0.515).
    // With is_higher_multiplier_excepted: true (school, hospital, surgery, library, court, etc.),
    // it must pay STANDARD multiplier (0.502).
    let mut req_higher = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "096".to_string(),
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
            apply_transitional_relief: Some(false),
            is_higher_multiplier_excepted: false,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res_higher = engine.calculate(req_higher.clone());
    let seg_higher = &res_higher.annual_billing_segments[0];
    assert_eq!(seg_higher.multiplier_used.multiplier_type, "HIGHER");
    assert_eq!(seg_higher.multiplier_used.rate, dec!(0.515));
    assert_eq!(seg_higher.gross_liability, dec!(77250.00));

    // Now test with exception
    req_higher.qualifying_relief_flags.is_higher_multiplier_excepted = true;
    let res_excepted = engine.calculate(req_higher);
    let seg_excepted = &res_excepted.annual_billing_segments[0];
    assert_eq!(seg_excepted.multiplier_used.multiplier_type, "STANDARD");
    assert_eq!(seg_excepted.multiplier_used.rate, dec!(0.502));
    assert_eq!(seg_excepted.gross_liability, dec!(75300.00));
}

#[tokio::test]
async fn test_wales_sbrr_childcare_100_percent_up_to_100k() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #24: Welsh Childcare SBRR (100% relief up to £100k RV under W.S.I. 2017/1229 art. 8 / W.S.I. 2018/1193)
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "084",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 45000.00
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
            "is_registered_childcare": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    // Under W.S.I. 2017/1229 art. 8, £45,000 childcare receives 100% relief (£0 net payable)
    assert_eq!(seg.net_segment_payable, dec!(0.00));
    let sbrr = seg.itemized_reliefs.iter().find(|r| r.relief_code.contains("SBRR")).expect("Welsh SBRR should apply to childcare");
    assert_eq!(sbrr.discount_percentage, dec!(100.00));
    assert!(sbrr.statutory_authority.contains("2017/1229"));
}

#[tokio::test]
async fn test_wales_sbrr_post_office_tiered_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #24: Welsh Post Office SBRR: 100% up to £9,000 and 50% from £9,001 to £12,000 (W.S.I. 2017/1229 art. 9 & 11(2))
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "227",
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
            "is_small_business_sole_property": true,
            "is_post_office": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    // Under standard taper, £10,000 gets (12000-10000)/6000 = 33.33%. Under Post Office rule, it gets 50.00% (E = 2).
    let sbrr = seg.itemized_reliefs.iter().find(|r| r.relief_code.contains("SBRR")).expect("Welsh SBRR should apply to post office");
    assert_eq!(sbrr.discount_percentage, dec!(50.00));
}

#[tokio::test]
async fn test_wales_sbrr_two_properties_in_billing_authority() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #24: Two properties per billing authority under W.S.I. 2017/1229 art. 12
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "225",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 5000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_second_property_grace": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    let sbrr = seg.itemized_reliefs.iter().find(|r| r.relief_code.contains("SBRR")).expect("Second qualifying property in Wales must receive SBRR");
    assert_eq!(sbrr.discount_percentage, dec!(100.00));
    assert!(sbrr.statutory_authority.contains("art. 12"));
}

#[tokio::test]
async fn test_wales_sbrr_excepted_hereditaments_no_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #24: Excepted hereditaments (car parks SCAT 050, advert 003, beach huts 024) get no relief under W.S.I. 2017/1229 arts. 2 & 7(b)
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "050",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 5000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_small_business_sole_property": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code.contains("SBRR")),
        "Car park is an excepted hereditament under W.S.I. 2017/1229 art. 2 & 7(b) and must receive 0% Welsh SBRR");
}

#[tokio::test]
async fn test_wales_sbrr_part_occupied_excluded() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #24: Welsh SBRR requires occupation of the whole hereditament (arts. 7(c), 8(c), 9(c)).
    // Part-occupied (s.44A) apportionment properties are excluded from Welsh SBRR.
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "225",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 5000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "PART_OCCUPIED",
        "part_occupied_apportionment": {
            "occupied_rateable_value": 3000.00,
            "unoccupied_rateable_value": 2000.00
        },
        "qualifying_relief_flags": {
            "is_small_business_sole_property": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code.contains("SBRR")),
        "Part-occupied property in Wales must not receive SBRR under W.S.I. 2017/1229 art. 7(c)");
}

#[tokio::test]
async fn test_wales_empty_property_low_value_threshold_boundaries() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // 1. Boundary below threshold: RV £2,000 (< £2,200) qualifies for 100% low-value exemption under W.S.I. 2008/2499 reg. 4(g)
    let json_exempt = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 2000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "UNOCCUPIED",
        "qualifying_relief_flags": {}
    });

    let req_exempt: LiabilityCalculationRequest = serde_json::from_value(json_exempt).unwrap();
    let res_exempt = engine.calculate(req_exempt);
    let seg_exempt = &res_exempt.annual_billing_segments[0];
    let relief = seg_exempt.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_LOW_VALUE");
    assert!(relief.is_some(), "Wales empty property with RV £2,000 must receive low-value exemption");
    assert_eq!(
        relief.unwrap().statutory_authority,
        "LGFA 1988 s.45 / W.S.I. 2008/2499 reg. 4(g)"
    );
    assert_eq!(seg_exempt.net_segment_payable, dec!(0.00));

    // 2. Boundary above threshold: RV £2,500 (between Welsh £2,200 and English £2,900) is NOT exempt in Wales
    let json_non_exempt = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 2500.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "UNOCCUPIED",
        "qualifying_relief_flags": {}
    });

    let req_non_exempt: LiabilityCalculationRequest = serde_json::from_value(json_non_exempt).unwrap();
    let res_non_exempt = engine.calculate(req_non_exempt);
    let seg_non_exempt = &res_non_exempt.annual_billing_segments[0];
    assert!(
        !seg_non_exempt.itemized_reliefs.iter().any(|r| r.relief_code == "EMPTY_LOW_VALUE"),
        "Wales empty property with RV £2,500 must not receive low-value exemption (< £2,200)"
    );
    // 2500 * 0.562 / 365 * 274 = £1,054.49 payable
    assert!(seg_non_exempt.net_segment_payable > dec!(1000.00));
}

#[tokio::test]
async fn test_wales_telecoms_infrastructure_relief_not_granted() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Wales 2024/25 with is_telecommunications_infrastructure.
    // Wales has no statutory regulations under the 2018 Act, so no relief can be granted.
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 30000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_telecommunications_infrastructure": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code == "TELECOM_FIBRE_RELIEF"),
        "Telecoms infrastructure relief must not be granted in Wales"
    );
    // 30,000 * 0.562 = £16,860.00
    assert_eq!(seg.net_segment_payable, dec!(16860.00));
}

#[test]
fn test_welsh_artificial_avoidance_penalty_charged_once_across_segments() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 50000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2026-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_artificial_avoidance_suspected": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);

    assert_eq!(response.annual_billing_segments.len(), 2);
    // £500 + 3% of £50,000 = £500 + £1,500 = £2,000
    assert_eq!(
        response.summary_financials.total_statutory_penalties,
        Some(dec!(2000.00)),
        "Welsh artificial avoidance penalty should only be charged once per determination (£2,000), not per segment"
    );
}

#[tokio::test]
async fn test_wales_petrol_station_qualifies_for_rhl_2023_24_75_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "211",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 50000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2023-04-01",
            "end_date": "2024-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_rhl_qualifying": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    
    // Multiplier 2023/24 in Wales is 0.535 -> Gross = 50000 * 0.535 = 26750.00
    assert_eq!(seg.multiplier_used.rate, dec!(0.535));
    assert_eq!(seg.gross_liability, dec!(26750.00));
    
    // 2023/24 RLHRR is 75% -> 75% of 26750.00 = 20062.50
    let rhl = seg.itemized_reliefs.iter().find(|r| r.relief_code == "RHL_WALES_2023");
    assert!(rhl.is_some(), "Petrol filling station (SCAT 211) in Wales must receive RLHRR in 2023/24");
    assert_eq!(rhl.unwrap().amount_deducted, dec!(20062.50));
    assert_eq!(seg.net_segment_payable, dec!(6687.50));
}

#[tokio::test]
async fn test_wales_petrol_station_qualifies_for_rhl_2024_25_40_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "211",
            "rateable_value_history": [
                {
                    "effective_from": "2024-04-01",
                    "effective_to": null,
                    "rateable_value": 50000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_rhl_qualifying": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    
    // Multiplier 2024/25 in Wales is 0.562 -> Gross = 50000 * 0.562 = 28100.00
    assert_eq!(seg.multiplier_used.rate, dec!(0.562));
    assert_eq!(seg.gross_liability, dec!(28100.00));
    
    // 2024/25 RLHRR is 40% -> 40% of 28100.00 = 11240.00
    let rhl = seg.itemized_reliefs.iter().find(|r| r.relief_code == "RHL_WALES_2024");
    assert!(rhl.is_some(), "Petrol filling station (SCAT 211) in Wales must receive RLHRR in 2024/25");
    assert_eq!(rhl.unwrap().amount_deducted, dec!(11240.00));
    assert_eq!(seg.net_segment_payable, dec!(16860.00));
}

#[tokio::test]
async fn test_sector_relief_wales_2026_food_drink_hospitality() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // Case A: Welsh pub/restaurant (SCAT 209, RV £25,000) in 2026/27 receives 15% Food & Drink Hospitality relief
    let request_pub = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "209".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(25000.00),
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

    let response_pub = engine.calculate(request_pub);
    let seg = &response_pub.annual_billing_segments[0];
    let rhl = seg.itemized_reliefs.iter().find(|r| r.relief_code == "RHL_WALES_2026");
    assert!(rhl.is_some(), "Expected Welsh Food & Drink Hospitality relief in 2026/27");
    assert_eq!(rhl.unwrap().discount_percentage, dec!(15.00));
    assert_eq!(rhl.unwrap().relief_name, "Welsh Food & Drink Hospitality Relief");

    // Case B: Petrol filling station (SCAT 211) in Wales is explicitly excluded
    let request_petrol = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "211".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(25000.00),
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

    let response_petrol = engine.calculate(request_petrol);
    let seg_p = &response_petrol.annual_billing_segments[0];
    assert!(!seg_p.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")),
        "Petrol filling station in Wales must be excluded from RHL relief");
}

#[tokio::test]
async fn test_wales_holiday_let_excluded_from_2026_27_food_drink_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 50000.00
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
    
    // SCAT 249 is holiday accommodation, excluded from Food & Drink Hospitality relief
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")),
        "Holiday let (SCAT 249) in Wales must NOT receive 2026/27 Food and Drink Hospitality relief"
    );
    assert_eq!(seg.net_segment_payable, dec!(25100.00)); // 50000 * 0.502
}

#[tokio::test]
async fn test_wales_retail_shop_on_retail_multiplier_excluded_from_2026_27_food_drink_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
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
            "is_retail_premises": true,
            "is_rhl_qualifying": true,
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    
    // Retail shop pays retail multiplier 0.350
    assert_eq!(seg.multiplier_used.rate, dec!(0.350));
    assert_eq!(seg.multiplier_used.multiplier_type, "RETAIL");
    
    // Properties subject to the retail multiplier must not receive Food and Drink relief
    assert!(
        !seg.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")),
        "Properties subject to Welsh retail multiplier must NOT receive Food & Drink Hospitality relief"
    );
}

#[tokio::test]
async fn test_wales_transitional_relief_2026_year_1_67_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Baseline 2025/26: RV £4,000 * 0.568 = £2,272.00
    // 2026/27: RV £10,000 * 0.502 = £5,020.00
    // Increase = £5,020.00 - £2,272.00 = £2,748.00 (> £300 statutory threshold)
    // Year 1 (2026/27) Welsh TR = 67% of £2,748.00 = £1,841.16 (W.S.I. 2025/1371)
    let req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2026, 3, 31).unwrap()),
                    rateable_value: dec!(4000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(10000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(true),
            is_small_business_sole_property: false,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res = engine.calculate(req);
    let seg = &res.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(5020.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(1841.16));
    assert_eq!(seg.net_segment_payable, dec!(3178.84));
}

#[tokio::test]
async fn test_wales_transitional_relief_2027_year_2_34_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Baseline 2025/26: RV £4,000 * 0.568 = £2,272.00
    // 2027/28: RV £10,000 * 0.502 = £5,020.00
    // Increase = £5,020.00 - £2,272.00 = £2,748.00 (> £300 statutory threshold)
    // Year 2 (2027/28) Welsh TR = 34% of £2,748.00 = £934.32 (W.S.I. 2025/1371)
    let req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2027, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2028, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2026, 3, 31).unwrap()),
                    rateable_value: dec!(4000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(10000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(true),
            is_small_business_sole_property: false,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res = engine.calculate(req);
    let seg = &res.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(5020.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(934.32));
    assert_eq!(seg.net_segment_payable, dec!(4085.68));
}

#[tokio::test]
async fn test_wales_transitional_relief_under_300_no_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Baseline 2025/26: RV £8,600 * 0.568 = £4,884.80
    // 2026/27: RV £10,000 * 0.502 = £5,020.00
    // Increase = £5,020.00 - £4,884.80 = £135.20 (<= £300 statutory threshold)
    // No Welsh TR awarded under W.S.I. 2025/1371
    let req = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2027, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::WALES,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2026, 3, 31).unwrap()),
                    rateable_value: dec!(8600.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(10000.00),
                },
            ],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            apply_transitional_relief: Some(true),
            is_small_business_sole_property: false,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let res = engine.calculate(req);
    let seg = &res.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(5020.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "NONE");
    assert_eq!(seg.transitional_adjustment.amount, dec!(0.00));
    assert_eq!(seg.net_segment_payable, dec!(5020.00));
}

#[tokio::test]
async fn test_wales_transitional_relief_excluded_when_occupier_changed() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reg 6(b),(c) of W.S.I. 2025/1371: Change of occupier ends transitional relief
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": "2026-03-31",
                    "rateable_value": 4000.00
                },
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 10000.00
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
            "same_occupier_since_revaluation": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.transitional_adjustment.amount, dec!(0.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "NONE");
}

#[tokio::test]
async fn test_wales_transitional_relief_excluded_for_part_occupied_hereditament() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reg 6(d) of W.S.I. 2025/1371: Section 44A apportionment excluded from Welsh TR
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": "2026-03-31",
                    "rateable_value": 4000.00
                },
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 10000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2026-04-01",
            "end_date": "2027-03-31"
        },
        "occupancy_type": "PART_OCCUPIED",
        "occupancy_periods": [
            {
                "start_date": "2026-04-01",
                "end_date": "2027-03-31",
                "occupancy_type": "PART_OCCUPIED",
                "part_occupied_apportionment": {
                    "occupied_percentage": 60.00
                }
            }
        ],
        "qualifying_relief_flags": {
            "apply_transitional_relief": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.transitional_adjustment.amount, dec!(0.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "NONE");
}

#[tokio::test]
async fn test_wales_transitional_relief_with_sbrr_on_both_sides() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // RV £8,000 on 31 March 2026 -> £11,000 on 1 April 2026
    // Sole small business occupier
    // Year 1 (2026/27): 67% of (NCA - BL)
    // BL: £8,000 * 0.568 * (1/3) = £1,514.67 (Welsh SBRR 66.6667% taper on £8k)
    // NCA: £11,000 * 0.502 * (5/6) = £4,601.67 (Welsh SBRR 16.6667% taper on £11k)
    // Increase = £4,601.67 - £1,514.67 = £3,087.00 (> £300)
    // Welsh TR = 67% of £3,087.00 = £2,068.29
    // 2026/27 Gross = £11,000 * 0.502 = £5,522.00
    // SBRR in 2026/27 = £920.33
    // Net payable = £5,522.00 - £920.33 - £2,068.29 = £2,533.38
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": "2026-03-31",
                    "rateable_value": 8000.00
                },
                {
                    "effective_from": "2026-04-01",
                    "effective_to": null,
                    "rateable_value": 11000.00
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
            "is_small_business_sole_property": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(5522.00));
    assert_eq!(seg.transitional_adjustment.amount, dec!(2068.26));
    assert_eq!(seg.net_segment_payable, dec!(2533.22));
}

#[tokio::test]
async fn test_wales_2023_list_transitional_relief_2024_25_uses_2023_multiplier() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // RV £20,000 in 2022/23 -> £40,000 on 1 April 2023
    // Year being calculated: 2024/25
    // Under W.S.I. 2022/1350:
    // BL (31 March 2023): £20,000 * 0.535 = £10,700.00
    // NCA (1 April 2023): £40,000 * 0.535 (2023/24 multiplier) = £21,400.00
    // Increase = £21,400.00 - £10,700.00 = £10,700.00
    // Year 2 (2024/25) TR = 34% of £10,700.00 = £3,638.00
    // 2024/25 Gross = £40,000 * 0.562 = £22,480.00
    // Net payable = £22,480.00 - £3,638.00 = £18,842.00
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2020-04-01",
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
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": true,
            "is_small_business_sole_property": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.gross_liability, dec!(22480.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(3638.00));
    assert_eq!(seg.net_segment_payable, dec!(18842.00));
}

#[tokio::test]
async fn test_wales_2017_list_small_business_relief_75_50_25_phasing() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // W.S.I. 2016/1247: Small business occupier with RV £4,000 on 31 March 2017 -> £10,000 on 1 April 2017
    // 31 March 2017: RV £4,000 qualifies for 100% SBRR -> BL = £0.00
    // 1 April 2017: RV £10,000 * 0.499 = £4,990.00 gross; Welsh SBRR taper (33.33% relief) = £1,663.17
    // NCA = £4,990.00 - £1,663.17 = £3,326.83
    // NCA exceeds BL + £100 (threshold under reg 9(b))
    // 2017/18: 75% relief = £3,326.83 * 0.75 = £2,495.12
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2010-04-01",
                    "effective_to": "2017-03-31",
                    "rateable_value": 4000.00
                },
                {
                    "effective_from": "2017-04-01",
                    "effective_to": null,
                    "rateable_value": 10000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2017-04-01",
            "end_date": "2020-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": true,
            "is_small_business_sole_property": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    assert_eq!(response.annual_billing_segments.len(), 3);

    // 2017/18: 75% relief
    let seg1 = &response.annual_billing_segments[0];
    assert_eq!(seg1.financial_year, "2017/18");
    assert_eq!(seg1.transitional_adjustment.amount, dec!(2495.12));
    assert_eq!(seg1.net_segment_payable, dec!(831.71));

    // 2018/19: 50% relief
    let seg2 = &response.annual_billing_segments[1];
    assert_eq!(seg2.financial_year, "2018/19");
    assert_eq!(seg2.transitional_adjustment.amount, dec!(1663.42));

    // 2019/20: 25% relief
    let seg3 = &response.annual_billing_segments[2];
    assert_eq!(seg3.financial_year, "2019/20");
    assert_eq!(seg3.transitional_adjustment.amount, dec!(831.71));
}

#[tokio::test]
async fn test_wales_2017_list_no_small_business_relief_gets_zero_transitional_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #23: Wales, RV £20,000 (2010 list) -> £40,000 (2017 list).
    // Under W.S.I. 2016/1247 reg 9(a), property must have been receiving SBRR on 31 March 2017.
    // Since RV £20,000 does not qualify for SBRR, no transitional relief is awarded.
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2010-04-01",
                    "effective_to": "2017-03-31",
                    "rateable_value": 20000.00
                },
                {
                    "effective_from": "2017-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2017-04-01",
            "end_date": "2018-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": true,
            "is_small_business_sole_property": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.transitional_adjustment.amount, dec!(0.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "NONE");
}

#[tokio::test]
async fn test_wales_2005_list_no_transitional_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #23: Wales had no transitional relief scheme for the 2005 rating list
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2000-04-01",
                    "effective_to": "2005-03-31",
                    "rateable_value": 20000.00
                },
                {
                    "effective_from": "2005-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2005-04-01",
            "end_date": "2006-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.transitional_adjustment.amount, dec!(0.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "NONE");
}

#[tokio::test]
async fn test_wales_2010_list_no_transitional_relief() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Reproduction for Issue #23: Wales had no transitional relief scheme for the 2010 rating list
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2005-04-01",
                    "effective_to": "2010-03-31",
                    "rateable_value": 20000.00
                },
                {
                    "effective_from": "2010-04-01",
                    "effective_to": null,
                    "rateable_value": 40000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2010-04-01",
            "end_date": "2011-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": true
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];

    assert_eq!(seg.transitional_adjustment.amount, dec!(0.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "NONE");
}
