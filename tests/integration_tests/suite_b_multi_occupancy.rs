//! Suite B: Multi-Occupancy, Mid-Year RV Slicing & Period Validation
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
async fn test_multi_occupancy_api_endpoint_json_serialization() {
    let app = create_test_app();

    let json_body = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "rateable_value": 50000.0
                }
            ]
        },
        "occupancy_periods": [
            {
                "start_date": "2024-04-01",
                "end_date": "2024-06-30",
                "occupancy_type": "UNOCCUPIED"
            },
            {
                "start_date": "2024-07-01",
                "end_date": "2025-03-31",
                "occupancy_type": "OCCUPIED"
            }
        ]
    });

    let req = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-API-Key", "br_live_mcc_key_12345")
        .body(Body::from(serde_json::to_vec(&json_body).unwrap()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&bytes).unwrap();

    let segments = body["annual_billing_segments"].as_array().unwrap();
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0]["occupancy_type"], "UNOCCUPIED");
    assert_eq!(segments[1]["occupancy_type"], "OCCUPIED");
}

#[test]
fn test_mid_year_rateable_value_change_splits_billing_segments() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": "2024-09-30",
                    "rateable_value": 20000.00
                },
                {
                    "effective_from": "2024-10-01",
                    "effective_to": null,
                    "rateable_value": 60000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2024-04-01",
            "end_date": "2025-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let response = engine.calculate(request);

    // Must be split into 2 segments within 2024/25:
    // Seg 1: 2024-04-01 to 2024-09-30 (183 days) @ RV £20,000
    // Seg 2: 2024-10-01 to 2025-03-31 (182 days) @ RV £60,000
    assert_eq!(response.annual_billing_segments.len(), 2);
    let seg1 = &response.annual_billing_segments[0];
    let seg2 = &response.annual_billing_segments[1];

    assert_eq!(seg1.rateable_value, dec!(20000.00));
    assert_eq!(seg1.segment_period.days, 183);
    assert_eq!(seg2.rateable_value, dec!(60000.00));
    assert_eq!(seg2.segment_period.days, 182);

    // Multiplier for England 2024/25:
    // Seg 1 at RV £20,000 -> Small Business Multiplier (0.499) -> (20,000 * 0.499) / 365 * 183 = £5,003.67
    // Seg 2 at RV £60,000 -> Standard Multiplier (0.546) -> (60,000 * 0.546) / 365 * 182 = £16,335.12
    assert_eq!(seg1.gross_liability, dec!(5003.67));
    assert_eq!(seg2.gross_liability, dec!(16335.12));
}

#[tokio::test]
async fn test_calculate_liability_period_before_first_rv_returns_bad_request() {
    let app = create_test_app();
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [
                {
                    "effective_from": "2024-04-01",
                    "effective_to": null,
                    "rateable_value": 20000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2023-04-01",
            "end_date": "2024-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {}
    });

    let req = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&json).unwrap()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let body = body_to_string(res.into_body()).await;
    let err_json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(err_json["error"], "RATEABLE_VALUE_MISSING_FOR_PERIOD");
}

#[tokio::test]
async fn test_overlapping_occupancy_periods_rejected() {
    let app = create_test_app();
    let body = |periods: Value| serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "249",
            "rateable_value_history": [{"effective_from": "2023-04-01", "rateable_value": 50000.0}]
        },
        "occupancy_periods": periods,
        "qualifying_relief_flags": {}
    });

    for periods in [
        // Exact duplicate
        serde_json::json!([
            {"start_date": "2024-04-01", "end_date": "2025-03-31", "occupancy_type": "OCCUPIED"},
            {"start_date": "2024-04-01", "end_date": "2025-03-31", "occupancy_type": "OCCUPIED"}
        ]),
        // Listed out of order, sharing one day (end dates are inclusive)
        serde_json::json!([
            {"start_date": "2024-10-01", "end_date": "2025-03-31", "occupancy_type": "OCCUPIED"},
            {"start_date": "2024-04-01", "end_date": "2024-10-01", "occupancy_type": "UNOCCUPIED"}
        ]),
    ] {
        let req = Request::builder()
            .uri("/api/v1/calculate-liability?lang=cy")
            .method("POST")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_vec(&body(periods)).unwrap()))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let json: Value = serde_json::from_str(&body_to_string(res.into_body()).await).unwrap();
        assert_eq!(json["error"], "OVERLAPPING_OCCUPATION_PERIODS");
        assert!(json["message"].as_str().unwrap().contains("gorgyffwrdd"));
    }

    // Contiguous periods are still accepted
    let req = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&body(serde_json::json!([
            {"start_date": "2024-04-01", "end_date": "2024-09-30", "occupancy_type": "UNOCCUPIED"},
            {"start_date": "2024-10-01", "end_date": "2025-03-31", "occupancy_type": "OCCUPIED"}
        ]))).unwrap()))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

// =========================================================================
// SUITE D: STATUTORY CALCULATION ENGINE COMPREHENSIVE COVERAGE
// =========================================================================

#[tokio::test]
async fn test_open_ended_occupation_empty_string_and_null() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1. Test empty string end_date (exactly what user sent)
    let json_empty_string = r#"{
        "party_type": "CHARITY",
        "occupation_period": {
            "start_date": "2023-04-01",
            "end_date": ""
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "211",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": "",
                    "rateable_value": 14500
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_charitable_purposes": true
        }
    }"#;

    let req: LiabilityCalculationRequest = serde_json::from_str(json_empty_string)
        .expect("Empty string end_date and effective_to must deserialize cleanly");
    assert_eq!(req.occupation_period.end_date, NaiveDate::from_ymd_opt(2027, 3, 31).unwrap());

    let res = engine.calculate(req);
    assert_eq!(res.annual_billing_segments.len(), 4); // 2023/24, 2024/25, 2025/26, 2026/27
    assert_eq!(res.occupation_period.end_date, NaiveDate::from_ymd_opt(2027, 3, 31).unwrap());

    // 2. Test null end_date
    let json_null = r#"{
        "party_type": "CHARITY",
        "occupation_period": {
            "start_date": "2025-04-01",
            "end_date": null
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "211",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 14500
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_charitable_purposes": true
        }
    }"#;

    let req_null: LiabilityCalculationRequest = serde_json::from_str(json_null)
        .expect("Null end_date must deserialize cleanly");
    assert_eq!(req_null.occupation_period.end_date, NaiveDate::from_ymd_opt(2027, 3, 31).unwrap());

    // 3. Test omitted end_date
    let json_omitted = r#"{
        "party_type": "CHARITY",
        "occupation_period": {
            "start_date": "2025-04-01"
        },
        "hereditament": {
            "jurisdiction": "ENGLAND",
            "scat_code": "211",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "rateable_value": 14500
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_charitable_purposes": true
        }
    }"#;

    let req_omitted: LiabilityCalculationRequest = serde_json::from_str(json_omitted)
        .expect("Omitted end_date must deserialize cleanly with default");
    assert_eq!(req_omitted.occupation_period.end_date, NaiveDate::from_ymd_opt(2027, 3, 31).unwrap());
}

#[tokio::test]
async fn test_empty_property_prior_unoccupied_days_out_of_order_periods() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // List periods out of order:
    // Period 1 (listed first): 2025/26 UNOCCUPIED with days_previously_unoccupied = 91
    // Period 2 (listed second): 2024/25 UNOCCUPIED with days_previously_unoccupied = 0
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(), // 91 days max exemption
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: Some(vec![
            OccupancyPeriodInput {
                start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(), // 91 days
                occupancy_type: OccupancyType::UNOCCUPIED,
                qualifying_relief_flags: Some(QualifyingReliefFlags {
                    days_previously_unoccupied: Some(91),
                    ..Default::default()
                }),
                part_occupied_apportionment: None,
            },
            OccupancyPeriodInput {
                start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(), // 91 days
                occupancy_type: OccupancyType::UNOCCUPIED,
                qualifying_relief_flags: Some(QualifyingReliefFlags {
                    days_previously_unoccupied: Some(0),
                    ..Default::default()
                }),
                part_occupied_apportionment: None,
            },
        ]),
        language: None,
    };

    let response = engine.calculate(request);
    // Chronologically first segment should be 2024/25, which should get full 91 days exemption (net = 0)
    assert_eq!(response.annual_billing_segments[0].financial_year, "2024/25");
    let r2024 = response.annual_billing_segments[0]
        .itemized_reliefs
        .iter()
        .find(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION");
    assert!(r2024.is_some(), "2024/25 should receive empty property exemption");
    assert_eq!(r2024.unwrap().discount_percentage, dec!(100.00));
    assert_eq!(response.annual_billing_segments[0].net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_unseeded_year_raises_error_not_045_default() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // A calculation for a year without seeded multiplier (e.g. 2028/29)
    // Must return an Err(AppError::MissingMultiplierForPeriod), NOT silently calculate with 0.450
    let json = serde_json::json!({
        "party_type": "CORPORATE",
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2028-04-01",
                    "effective_to": null,
                    "rateable_value": 50000.00
                }
            ]
        },
        "occupation_period": {
            "start_date": "2028-04-01",
            "end_date": "2029-03-31"
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "apply_transitional_relief": false
        }
    });

    let request: LiabilityCalculationRequest = serde_json::from_value(json).unwrap();
    let result = engine.try_calculate(request);
    assert!(result.is_err(), "Calculation for unseeded 2028/29 must return Err, not silent 0.45 default");
    match result.unwrap_err() {
        AppError::MissingMultiplierForPeriod { financial_year, jurisdiction } => {
            assert_eq!(financial_year, "2028/29");
            assert_eq!(jurisdiction, Jurisdiction::WALES);
        }
        other => panic!("Expected MissingMultiplierForPeriod error, got: {:?}", other),
    }
}
