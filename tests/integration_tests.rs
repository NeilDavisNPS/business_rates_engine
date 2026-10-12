use rust_decimal_macros::dec;
use std::fs;
use std::path::PathBuf;

use business_rates_engine::models::calculation::*;
use business_rates_engine::models::enums::{Jurisdiction, PartyType, OccupancyType};
use business_rates_engine::engine::calculator::CalculationEngine;
use business_rates_engine::db::repository::ParameterRepository;
use business_rates_engine::models::AppState;
use business_rates_engine::api::create_router;
use business_rates_engine::api::handlers::resolve_language;
use business_rates_engine::i18n::Language;
use business_rates_engine::error::AppError;
use chrono::NaiveDate;
use axum::body::Body;
use axum::http::{header, Request, StatusCode, HeaderMap};
use http_body_util::BodyExt;
use tower::ServiceExt;
use serde_json::Value;

fn create_test_app() -> axum::Router {
    let repo = ParameterRepository::new_mock();
    let state = AppState { repo };
    create_router(state)
}

async fn body_to_string(body: Body) -> String {
    let bytes = body.collect().await.expect("Failed to read response body").to_bytes();
    String::from_utf8(bytes.to_vec()).expect("Response body is not valid UTF-8")
}

fn load_fixture(filename: &str) -> LiabilityCalculationRequest {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(filename);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read fixture at {:?}", path));
    serde_json::from_str(&content).expect("Failed to deserialize fixture request JSON")
}


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
    assert_eq!(seg2.multiplier_used.rate, dec!(0.522));
    assert!(seg2.itemized_reliefs.iter().any(|r| r.relief_code == "RHL_2026"));

    assert!(response.calculation_narrative.contains("Local Government Finance Act 1988"));
}

#[tokio::test]
async fn test_scenario_2_wales_petrol_station() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);
    let request = load_fixture("scenario_2_wales_petrol_station.json");

    let response = engine.calculate(request);
    assert_eq!(response.jurisdiction, Jurisdiction::WALES);
    assert_eq!(response.annual_billing_segments[0].multiplier_used.rate, dec!(0.562));
    assert_eq!(response.annual_billing_segments[0].multiplier_used.multiplier_type, "WELSH_DIFFERENTIAL");
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
    assert_eq!(response.annual_billing_segments[0].multiplier_used.rate, dec!(0.568));
}

#[tokio::test]
async fn test_zero_hardcoding_historical_multipliers_1990_to_2026() {
    let repo = ParameterRepository::new_seeded();

    // 1990/91 Statutory Multipliers (Inaugural NNDR under LGFA 1988)
    let m_90_eng = repo.resolve_multiplier(Jurisdiction::ENGLAND, "1990/91", dec!(25000.00));
    assert_eq!(m_90_eng.rate, dec!(0.348));

    let m_90_wal = repo.resolve_multiplier(Jurisdiction::WALES, "1990/91", dec!(25000.00));
    assert_eq!(m_90_wal.rate, dec!(0.368));

    let m_90_sco = repo.resolve_multiplier(Jurisdiction::SCOTLAND, "1990/91", dec!(25000.00));
    assert_eq!(m_90_sco.rate, dec!(0.410));

    // 1993/94 Statutory Multipliers
    let m_93_eng = repo.resolve_multiplier(Jurisdiction::ENGLAND, "1993/94", dec!(25000.00));
    assert_eq!(m_93_eng.rate, dec!(0.416));

    let m_93_sco = repo.resolve_multiplier(Jurisdiction::SCOTLAND, "1993/94", dec!(25000.00));
    assert_eq!(m_93_sco.rate, dec!(0.477));

    // 2000/01 Revaluation List
    let m_00_eng = repo.resolve_multiplier(Jurisdiction::ENGLAND, "2000/01", dec!(10000.00));
    assert_eq!(m_00_eng.rate, dec!(0.416));

    // 2010/11 Revaluation List (Small Business vs Standard)
    let m_10_small = repo.resolve_multiplier(Jurisdiction::ENGLAND, "2010/11", dec!(12000.00));
    assert_eq!(m_10_small.rate, dec!(0.407));
    assert_eq!(m_10_small.multiplier_type, "SMALL_BUSINESS");

    let m_10_std = repo.resolve_multiplier(Jurisdiction::ENGLAND, "2010/11", dec!(60000.00));
    assert_eq!(m_10_std.rate, dec!(0.414));
    assert_eq!(m_10_std.multiplier_type, "STANDARD");

    // 2026/27 High-Value Threshold (£500k)
    let m_26_hv = repo.resolve_multiplier(Jurisdiction::ENGLAND, "2026/27", dec!(500000.00));
    assert_eq!(m_26_hv.rate, dec!(0.568));
    assert_eq!(m_26_hv.multiplier_type, "HIGH_VALUE");
}

#[tokio::test]
async fn test_mandatory_charitable_relief_80_percent() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CHARITY,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_charitable_purposes: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let charity_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(charity_relief.discount_percentage, dec!(80.00));
}

#[tokio::test]
async fn test_ndra_2023_improvement_relief_g_factor() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "249".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(30000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            qualifying_improvement_rv_increment: dec!(5000.00),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let g_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "IMPROVEMENT_G_FACTOR").unwrap();
    // 5000 * 0.499 = 2495.00
    assert_eq!(g_relief.amount_deducted, dec!(2495.00));
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
async fn test_suite_8_empty_property_exemption() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Industrial warehouse (SCAT 096) unoccupied for 90 days (within 182-day exemption)
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 6, 29).unwrap(), // 90 days
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(50000.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_industrial_hereditament: true,
            days_previously_unoccupied: Some(0),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let empty_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION");
    assert!(empty_relief.is_some());
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_app_state_repository_access() {
    let state = AppState {
        repo: ParameterRepository::new_mock(),
    };
    assert!(!state.repo.all_reference_codes().is_empty());
}

#[tokio::test]
async fn test_preseeded_reference_codes_and_allowable_values() {
    let repo = ParameterRepository::new_mock();
    let codes = repo.all_reference_codes();
    assert!(!codes.is_empty(), "Reference codes must be pre-seeded");

    // Check categories
    let party_types = repo.reference_codes_by_category("PARTY_TYPE");
    assert!(party_types.iter().any(|c| c.code == "CORPORATE" && c.label.contains("Company")));
    assert!(party_types.iter().any(|c| c.code == "INDIVIDUAL"));
    assert!(party_types.iter().any(|c| c.code == "CHARITY"));

    let jurisdictions = repo.reference_codes_by_category("JURISDICTION");
    assert_eq!(jurisdictions.len(), 3);
    assert!(jurisdictions.iter().any(|c| c.code == "ENGLAND"));
    assert!(jurisdictions.iter().any(|c| c.code == "WALES"));
    assert!(jurisdictions.iter().any(|c| c.code == "SCOTLAND"));

    let scats = repo.reference_codes_by_category("SCAT_CODE");
    assert!(scats.iter().any(|c| c.code == "003" && c.label.contains("Airport")));
    assert!(scats.iter().any(|c| c.code == "211" && c.label.contains("Petrol")));
    assert!(scats.iter().any(|c| c.code == "249" && c.label.contains("Holiday")));

    let reliefs = repo.reference_codes_by_category("RELIEF_CODE");
    assert!(reliefs.iter().any(|c| c.code == "SBRR" && c.jurisdiction.as_deref() == Some("ENGLAND")));
    assert!(reliefs.iter().any(|c| c.code == "SBBS" && c.jurisdiction.as_deref() == Some("SCOTLAND")));
    assert!(reliefs.iter().any(|c| c.code == "RHL" && c.jurisdiction.as_deref() == Some("ENGLAND")));
    assert!(reliefs.iter().any(|c| c.code == "WELSH_AVOIDANCE" && c.jurisdiction.as_deref() == Some("WALES")));
}

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
async fn test_i18n_welsh_narrative_and_relief_names() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let json_welsh = r#"{
        "party_type": "CHARITY",
        "occupation_period": {
            "start_date": "2025-04-01",
            "end_date": "2026-03-31"
        },
        "hereditament": {
            "jurisdiction": "WALES",
            "scat_code": "096",
            "rateable_value_history": [
                {
                    "effective_from": "2023-04-01",
                    "effective_to": null,
                    "rateable_value": 20000.00
                }
            ]
        },
        "occupancy_type": "OCCUPIED",
        "qualifying_relief_flags": {
            "is_charitable_purposes": true
        },
        "language": "cy"
    }"#;

    let req: LiabilityCalculationRequest = serde_json::from_str(json_welsh).unwrap();
    assert_eq!(req.language.as_deref(), Some("cy"));

    let res = engine.calculate(req);

    // Verify Welsh narrative headers and lines
    assert!(res.calculation_narrative.contains("ADRODDIAD STATUDOL AR GYFRIFO TRETHI BUSNES & LLWYBR ARCHWILIO"));
    assert!(res.calculation_narrative.contains("1. Fframwaith Statudol:"));
    assert!(res.calculation_narrative.contains("Deddf Cyllid Llywodraeth Leol (Cymru) 2024"));
    assert!(res.calculation_narrative.contains("Blwyddyn Ariannol 2025/26:"));
    assert!(res.calculation_narrative.contains("Gwerth Trethiannol (RV):"));
    assert!(res.calculation_narrative.contains("Dyddiau Meddiannu:"));
    assert!(res.calculation_narrative.contains("Atebolrwydd Crynswth:"));
    assert!(res.calculation_narrative.contains("Swm Net y Segment i'w Dalu:"));
    assert!(res.calculation_narrative.contains("CYFANSWM Y SWM NET I'W DALU:"));

    // Verify Welsh localized relief name
    let seg = &res.annual_billing_segments[0];
    let charity_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(charity_relief.relief_name, "Rhyddhad Elusennol Gorfodol");
}

#[tokio::test]
async fn test_i18n_reference_codes_localized_welsh_and_english() {
    let repo = ParameterRepository::new_mock();

    // Welsh reference codes
    let cy_codes = repo.all_reference_codes_localized(business_rates_engine::i18n::Language::Cy);
    let england_cy = cy_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "ENGLAND").unwrap();
    assert_eq!(england_cy.label, "Lloegr");
    assert!(england_cy.description.contains("Ardrethu annomestig statudol"));

    let wales_cy = cy_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "WALES").unwrap();
    assert_eq!(wales_cy.label, "Cymru");

    let charity_cy = cy_codes.iter().find(|c| c.category == "PARTY_TYPE" && c.code == "CHARITY").unwrap();
    assert_eq!(charity_cy.label, "Elusen Gofrestredig");

    let occ_cy = cy_codes.iter().find(|c| c.category == "OCCUPANCY_TYPE" && c.code == "OCCUPIED").unwrap();
    assert_eq!(occ_cy.label, "Mangre a Feddienir");

    // English reference codes (default)
    let en_codes = repo.all_reference_codes_localized(business_rates_engine::i18n::Language::En);
    let england_en = en_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "ENGLAND").unwrap();
    assert_eq!(england_en.label, "England");

    let charity_en = en_codes.iter().find(|c| c.category == "PARTY_TYPE" && c.code == "CHARITY").unwrap();
    assert_eq!(charity_en.label, "Registered Charity");
}

#[tokio::test]
async fn test_i18n_localized_error_messages() {
    let err = business_rates_engine::error::AppError::InvalidOccupationPeriod {
        start: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
    };

    let msg_cy = err.localized_message(business_rates_engine::i18n::Language::Cy);
    assert_eq!(msg_cy, "Ni all dyddiad gorffen meddiannaeth (2024-03-31) fod cyn y dyddiad cychwyn (2025-04-01)");

    let msg_en = err.localized_message(business_rates_engine::i18n::Language::En);
    assert_eq!(msg_en, "Occupation end date (2024-03-31) cannot precede start date (2025-04-01)");
}

// =========================================================================
// SUITE A: UI ASKAMA HTML TEMPLATE RENDERING & FRONT-END STRUCTURE
// =========================================================================

#[tokio::test]
async fn test_ui_html_rendering_english_default() {
    let app = create_test_app();
    let req = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let html = body_to_string(res.into_body()).await;

    // Serves component Svelte 5 UI with div id="app" and matching favicon
    assert!(html.contains(r#"<div id="app"></div>"#));
    assert!(html.contains("<title>Business Rates Engine — Statutory Calculation Sandbox</title>"));
    assert!(html.contains(r#"<link rel="icon" type="image/svg+xml""#));
    assert!(html.contains("/assets/index-") && html.contains(".css"));
}

#[tokio::test]
async fn test_ui_decommissioned_alias_routes_return_404() {
    let app = create_test_app();
    for path in &["/admin", "/component-ui", "/app"] {
        let req = Request::builder()
            .uri(*path)
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND, "Decommissioned route {} must return 404", path);
    }
}

// =========================================================================
// SUITE C: HTTP REST API AUTHENTICATION & SECURITY
// =========================================================================

#[tokio::test]
async fn test_http_calculate_liability_unauthenticated_access() {
    let app = create_test_app();
    let req_fixture = load_fixture("scenario_1_sbrr_rhl_england.json");
    let json_body = serde_json::to_string(&req_fixture).unwrap();

    // 1. Missing X-API-Key header -> succeeds (unauthenticated API)
    let req_no_key = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json_body.clone()))
        .unwrap();

    let res = app.clone().oneshot(req_no_key).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body_str = body_to_string(res.into_body()).await;
    let json: Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(json["jurisdiction"], "ENGLAND");
    assert!(json["calculation_id"].is_string());
    assert_eq!(json["annual_billing_segments"].as_array().unwrap().len(), 2);

    // 2. Arbitrary X-API-Key header -> also succeeds cleanly
    let req_arbitrary_key = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-API-Key", "any_random_key_or_none")
        .body(Body::from(json_body))
        .unwrap();

    let res_arb = app.clone().oneshot(req_arbitrary_key).await.unwrap();
    assert_eq!(res_arb.status(), StatusCode::OK);

    // 3. Removed legacy /business-rates slug returns 404
    let req_legacy = Request::builder()
        .uri("/api/v1/business-rates/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{}"))
        .unwrap();
    let res_legacy = app.oneshot(req_legacy).await.unwrap();
    assert_eq!(res_legacy.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_http_calculate_liability_multilingual_content_negotiation() {
    let app = create_test_app();
    let req_fixture = load_fixture("scenario_1_sbrr_rhl_england.json");
    let json_body = serde_json::to_string(&req_fixture).unwrap();

    // 1. Language via query parameter ?lang=cy
    let req_query = Request::builder()
        .uri("/api/v1/calculate-liability?lang=cy")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-API-Key", "br_live_mcc_key_12345")
        .body(Body::from(json_body.clone()))
        .unwrap();

    let res_query = app.clone().oneshot(req_query).await.unwrap();
    assert_eq!(res_query.status(), StatusCode::OK);
    let body_query = body_to_string(res_query.into_body()).await;
    let json_query: Value = serde_json::from_str(&body_query).unwrap();
    let narrative_query = json_query["calculation_narrative"].as_str().unwrap();
    assert!(narrative_query.contains("ADRODDIAD STATUDOL AR GYFRIFO TRETHI BUSNES"));

    // 2. Language via Accept-Language header
    let req_header = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT_LANGUAGE, "cy")
        .header("X-API-Key", "br_live_mcc_key_12345")
        .body(Body::from(json_body))
        .unwrap();

    let res_header = app.clone().oneshot(req_header).await.unwrap();
    assert_eq!(res_header.status(), StatusCode::OK);
    let body_header = body_to_string(res_header.into_body()).await;
    let json_header: Value = serde_json::from_str(&body_header).unwrap();
    let narrative_header = json_header["calculation_narrative"].as_str().unwrap();
    assert!(narrative_header.contains("ADRODDIAD STATUDOL AR GYFRIFO TRETHI BUSNES"));

    // 3. Language via payload property "language": "cy"
    let mut req_payload = req_fixture.clone();
    req_payload.language = Some("cy".to_string());
    let payload_body = serde_json::to_string(&req_payload).unwrap();

    let req_prop = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-API-Key", "br_live_mcc_key_12345")
        .body(Body::from(payload_body))
        .unwrap();

    let res_prop = app.oneshot(req_prop).await.unwrap();
    assert_eq!(res_prop.status(), StatusCode::OK);
    let body_prop = body_to_string(res_prop.into_body()).await;
    let json_prop: Value = serde_json::from_str(&body_prop).unwrap();
    let narrative_prop = json_prop["calculation_narrative"].as_str().unwrap();
    assert!(narrative_prop.contains("ADRODDIAD STATUDOL AR GYFRIFO TRETHI BUSNES"));
}

#[tokio::test]
async fn test_http_calculate_liability_bad_payload_error() {
    let app = create_test_app();
    let malformed_json = r#"{"party_type": "CHARITY", "occupation_period": "#;

    let req = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-API-Key", "br_live_mcc_key_12345")
        .body(Body::from(malformed_json))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body_str = body_to_string(res.into_body()).await;
    let json: Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(json["error"], "PAYLOAD_DESERIALIZATION_ERROR");
}

#[tokio::test]
async fn test_http_calculate_liability_invalid_dates_localized() {
    let app = create_test_app();
    let mut req_fixture = load_fixture("scenario_1_sbrr_rhl_england.json");
    req_fixture.occupation_period.start_date = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();
    req_fixture.occupation_period.end_date = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();
    let json_body = serde_json::to_string(&req_fixture).unwrap();

    // 1. English localized error response
    let req_en = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-API-Key", "br_live_mcc_key_12345")
        .body(Body::from(json_body.clone()))
        .unwrap();

    let res_en = app.clone().oneshot(req_en).await.unwrap();
    assert_eq!(res_en.status(), StatusCode::BAD_REQUEST);
    let body_en = body_to_string(res_en.into_body()).await;
    let json_en: Value = serde_json::from_str(&body_en).unwrap();
    assert_eq!(json_en["error"], "INVALID_OCCUPATION_PERIOD");
    assert_eq!(
        json_en["message"],
        "Occupation end date (2024-03-31) cannot precede start date (2025-04-01)"
    );

    // 2. Welsh localized error response (?lang=cy)
    let req_cy = Request::builder()
        .uri("/api/v1/calculate-liability?lang=cy")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-API-Key", "br_live_mcc_key_12345")
        .body(Body::from(json_body))
        .unwrap();

    let res_cy = app.oneshot(req_cy).await.unwrap();
    assert_eq!(res_cy.status(), StatusCode::BAD_REQUEST);
    let body_cy = body_to_string(res_cy.into_body()).await;
    let json_cy: Value = serde_json::from_str(&body_cy).unwrap();
    assert_eq!(json_cy["error"], "INVALID_OCCUPATION_PERIOD");
    assert_eq!(
        json_cy["message"],
        "Ni all dyddiad gorffen meddiannaeth (2024-03-31) fod cyn y dyddiad cychwyn (2025-04-01)"
    );
}

#[tokio::test]
async fn test_http_reference_codes_endpoints() {
    let app = create_test_app();

    // 1. /api/v1/reference-codes is unauthenticated
    let req_pub = Request::builder()
        .uri("/api/v1/reference-codes")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_pub = app.clone().oneshot(req_pub).await.unwrap();
    assert_eq!(res_pub.status(), StatusCode::OK);
    let body_pub = body_to_string(res_pub.into_body()).await;
    assert!(body_pub.contains(r#""label":"England""#));

    // 2. Removed legacy /business-rates/reference-codes returns 404
    let req_legacy = Request::builder()
        .uri("/api/v1/business-rates/reference-codes")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_legacy = app.clone().oneshot(req_legacy).await.unwrap();
    assert_eq!(res_legacy.status(), StatusCode::NOT_FOUND);

    // 3. Endpoint localized in Welsh via query param
    let req_pub_cy = Request::builder()
        .uri("/api/v1/reference-codes?lang=cy")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_pub_cy = app.clone().oneshot(req_pub_cy).await.unwrap();
    assert_eq!(res_pub_cy.status(), StatusCode::OK);
    let body_pub_cy = body_to_string(res_pub_cy.into_body()).await;
    assert!(body_pub_cy.contains(r#""label":"Lloegr""#));
    assert!(body_pub_cy.contains(r#""label":"Cymru""#));

    // 4. Endpoint localized in Welsh via Accept-Language header
    let req_pub_hdr = Request::builder()
        .uri("/api/v1/reference-codes")
        .method("GET")
        .header(header::ACCEPT_LANGUAGE, "cy-GB,cy;q=0.9")
        .body(Body::empty())
        .unwrap();
    let res_pub_hdr = app.oneshot(req_pub_hdr).await.unwrap();
    assert_eq!(res_pub_hdr.status(), StatusCode::OK);
    let body_pub_hdr = body_to_string(res_pub_hdr.into_body()).await;
    assert!(body_pub_hdr.contains(r#""label":"Lloegr""#));
}

#[tokio::test]
async fn test_http_sso_endpoints() {
    let app = create_test_app();

    // SSO login endpoint has been removed -> returns 404
    let req_login = Request::builder()
        .uri("/api/v1/auth/sso/login")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_login = app.clone().oneshot(req_login).await.unwrap();
    assert_eq!(res_login.status(), StatusCode::NOT_FOUND);

    // SSO callback endpoint has been removed -> returns 404
    let req_cb = Request::builder()
        .uri("/api/v1/auth/sso/callback")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_cb = app.oneshot(req_cb).await.unwrap();
    assert_eq!(res_cb.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_language_resolution_precedence() {
    // 1. Payload language overrides query and header
    let mut h = HeaderMap::new();
    h.insert(header::ACCEPT_LANGUAGE, "cy".parse().unwrap());
    assert_eq!(
        resolve_language(&h, Some("cy"), Some("en")),
        Language::En
    );

    // 2. Query param overrides Accept-Language header
    let mut h2 = HeaderMap::new();
    h2.insert(header::ACCEPT_LANGUAGE, "en".parse().unwrap());
    assert_eq!(
        resolve_language(&h2, Some("cy"), None),
        Language::Cy
    );

    // 3. Accept-Language header applies when no payload or query language
    let mut h3 = HeaderMap::new();
    h3.insert(header::ACCEPT_LANGUAGE, "cy-GB,cy;q=0.9,en;q=0.8".parse().unwrap());
    assert_eq!(
        resolve_language(&h3, None, None),
        Language::Cy
    );

    // 4. Default fallback is English
    let h4 = HeaderMap::new();
    assert_eq!(
        resolve_language(&h4, None, None),
        Language::En
    );
}

// =========================================================================
// SUITE D: STATUTORY CALCULATION ENGINE COMPREHENSIVE COVERAGE
// =========================================================================

#[tokio::test]
async fn test_statutory_heat_network_exemption() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(40000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_heat_network: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let heat_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "HEAT_NETWORK");
    assert!(heat_relief.is_some());
    let r = heat_relief.unwrap();
    assert_eq!(r.statutory_authority, "Non-Domestic Rating Act 2023 s.2");
    assert_eq!(r.discount_percentage, dec!(100.00));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_statutory_public_lavatory_exemption() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let request = LiabilityCalculationRequest {
        party_type: PartyType::LOCAL_AUTHORITY,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(8000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_public_lavatory: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let lav_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "PUBLIC_LAVATORY");
    assert!(lav_relief.is_some());
    let r = lav_relief.unwrap();
    assert_eq!(r.statutory_authority, "Rating (Public Lavatories) Act 2021");
    assert_eq!(r.discount_percentage, dec!(100.00));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_statutory_rural_settlement_relief() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

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
                rateable_value: dec!(7500.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rural_settlement: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let rural_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "RURAL_SETTLEMENT");
    assert!(rural_relief.is_some());
    let r = rural_relief.unwrap();
    assert_eq!(r.statutory_authority, "LGFA 1988 s.43(6B) / LGFA 1997");
    assert_eq!(r.discount_percentage, dec!(100.00));
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
async fn test_statutory_empty_low_value_exemption() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // Empty property with RV £2,500 (< £2,900 statutory threshold under SI 2008/386)
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(2500.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags::default(),
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let empty_low = seg.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_LOW_VALUE");
    assert!(empty_low.is_some());
    assert_eq!(empty_low.unwrap().discount_percentage, dec!(100.00));
    assert_eq!(seg.net_segment_payable, dec!(0.00));
}

#[tokio::test]
async fn test_statutory_empty_property_prior_unoccupied_days() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // Commercial office (SCAT 091, 91 max exempt days)
    // 30 days already unoccupied before occupation start -> 61 remaining exempt days
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(), // 365 days
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_industrial_hereditament: false,
            days_previously_unoccupied: Some(30),
            ..Default::default()
        },
        occupancy_periods: None,
        language: None,
    };

    let response = engine.calculate(request);
    let seg = &response.annual_billing_segments[0];
    let empty_relief = seg.itemized_reliefs.iter().find(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION");
    assert!(empty_relief.is_some());
    let r = empty_relief.unwrap();
    // 61 days exempt out of 365 days: discount ratio 61 / 365 * 100% = 16.71%
    assert_eq!(r.discount_percentage, dec!(16.71));
    assert!(seg.net_segment_payable > dec!(0.00));
}

#[test]
fn test_domain_error_status_codes_and_codes() {
    let err_period = AppError::InvalidOccupationPeriod {
        start: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
    };
    assert_eq!(err_period.status_code(), StatusCode::BAD_REQUEST);
    assert_eq!(err_period.error_code(), "INVALID_OCCUPATION_PERIOD");

    let err_payload = AppError::PayloadDeserialization("invalid json syntax".to_string());
    assert_eq!(err_payload.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(err_payload.error_code(), "PAYLOAD_DESERIALIZATION_ERROR");
}

// =========================================================================
// SUITE E: OWASP RECOMMENDED SECURE HEADERS
// =========================================================================

#[tokio::test]
async fn test_owasp_secure_headers_on_html_ui_response() {
    let app = create_test_app();
    let req = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let headers = res.headers();

    // 1. Content-Security-Policy (CSP) - Strict OWASP policy with NO unsafe-inline
    let csp = headers.get(header::CONTENT_SECURITY_POLICY).expect("CSP header must be present");
    let csp_str = csp.to_str().unwrap();
    assert!(csp_str.contains("default-src 'self'"));
    assert!(!csp_str.contains("'unsafe-inline'"), "CSP must NOT contain unsafe-inline");
    assert!(csp_str.contains("script-src 'self'"));
    assert!(csp_str.contains("style-src 'self'"));
    assert!(csp_str.contains("frame-ancestors 'none'"));
    assert!(csp_str.contains("base-uri 'self'"));

    // 2. X-Content-Type-Options
    assert_eq!(headers.get(header::X_CONTENT_TYPE_OPTIONS).unwrap(), "nosniff");

    // 3. X-Frame-Options
    assert_eq!(headers.get(header::X_FRAME_OPTIONS).unwrap(), "DENY");

    // 4. Strict-Transport-Security (HSTS)
    assert_eq!(
        headers.get(header::STRICT_TRANSPORT_SECURITY).unwrap(),
        "max-age=31536000; includeSubDomains"
    );

    // 5. Referrer-Policy
    assert_eq!(
        headers.get(header::REFERRER_POLICY).unwrap(),
        "strict-origin-when-cross-origin"
    );

    // 6. Permissions-Policy
    assert_eq!(
        headers.get("permissions-policy").unwrap(),
        "camera=(), microphone=(), geolocation=(), payment=()"
    );

    // 7. Cross-Origin-Opener-Policy (COOP)
    assert_eq!(headers.get("cross-origin-opener-policy").unwrap(), "same-origin");

    // 8. Cross-Origin-Resource-Policy (CORP)
    assert_eq!(headers.get("cross-origin-resource-policy").unwrap(), "same-origin");

    // 9. X-Permitted-Cross-Domain-Policies
    assert_eq!(headers.get("x-permitted-cross-domain-policies").unwrap(), "none");
}

#[tokio::test]
async fn test_owasp_secure_headers_on_api_json_response() {
    let app = create_test_app();
    let req_fixture = load_fixture("scenario_1_sbrr_rhl_england.json");
    let json_body = serde_json::to_string(&req_fixture).unwrap();

    let req = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-API-Key", "br_live_mcc_key_12345")
        .body(Body::from(json_body))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let headers = res.headers();
    let csp = headers.get(header::CONTENT_SECURITY_POLICY).expect("CSP header must be present");
    let csp_str = csp.to_str().unwrap();
    assert!(!csp_str.contains("'unsafe-inline'"), "CSP on API responses must NOT contain unsafe-inline");
    assert!(csp_str.contains("script-src 'self'"));
    assert!(csp_str.contains("style-src 'self'"));
    assert_eq!(headers.get(header::X_CONTENT_TYPE_OPTIONS).unwrap(), "nosniff");
    assert_eq!(headers.get(header::X_FRAME_OPTIONS).unwrap(), "DENY");
    assert_eq!(headers.get(header::STRICT_TRANSPORT_SECURITY).unwrap(), "max-age=31536000; includeSubDomains");
    assert_eq!(headers.get(header::REFERRER_POLICY).unwrap(), "strict-origin-when-cross-origin");
    assert_eq!(headers.get("permissions-policy").unwrap(), "camera=(), microphone=(), geolocation=(), payment=()");
}

#[tokio::test]
async fn test_static_assets_served_with_correct_content_types() {
    let app = create_test_app();

    // 1. Removed static endpoints return 404 NOT FOUND (consolidated into Vite /assets/*)
    for path in ["/static/sandbox.js", "/static/sandbox.css", "/static/tailwind.min.css"] {
        let req = Request::builder()
            .uri(path)
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND, "Path {} must return 404", path);
    }
}

#[tokio::test]
async fn test_owasp_secure_headers_on_error_and_unauthorized_responses() {
    let app = create_test_app();

    // 1. Unauthenticated API returns 422 Unprocessable Entity on empty payload with OWASP headers
    let req_err = Request::builder()
        .uri("/api/v1/calculate-liability")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let res_err = app.clone().oneshot(req_err).await.unwrap();
    assert_eq!(res_err.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(res_err.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(), "nosniff");
    assert_eq!(res_err.headers().get(header::X_FRAME_OPTIONS).unwrap(), "DENY");
    assert!(res_err.headers().get(header::CONTENT_SECURITY_POLICY).is_some());

    // 2. Public reference codes 200 response
    let req_codes = Request::builder()
        .uri("/api/v1/reference-codes")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res_codes = app.oneshot(req_codes).await.unwrap();
    assert_eq!(res_codes.status(), StatusCode::OK);
    assert_eq!(res_codes.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(), "nosniff");
    assert_eq!(res_codes.headers().get("cross-origin-opener-policy").unwrap(), "same-origin");
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
    let request = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(12000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: Some(vec![
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
            OccupancyPeriodInput {
                start_date: NaiveDate::from_ymd_opt(2024, 7, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(), // 184 days
                occupancy_type: OccupancyType::OCCUPIED,
                qualifying_relief_flags: Some(QualifyingReliefFlags {
                    is_small_business_sole_property: true,
                    ..Default::default()
                }),
                part_occupied_apportionment: None,
            },
            OccupancyPeriodInput {
                start_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(), // 90 days
                occupancy_type: OccupancyType::PART_OCCUPIED,
                qualifying_relief_flags: Some(QualifyingReliefFlags {
                    is_small_business_sole_property: true,
                    ..Default::default()
                }),
                part_occupied_apportionment: Some(PartOccupiedApportionment {
                    occupied_rateable_value: Some(dec!(6000.00)),
                    unoccupied_rateable_value: Some(dec!(6000.00)),
                    occupied_percentage: Some(dec!(50.00)),
                }),
            },
        ]),
        language: None,
    };

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

#[tokio::test]
async fn test_i18n_scots_and_gaelic_narrative_and_relief_names() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    let req = LiabilityCalculationRequest {
        party_type: PartyType::CHARITY,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(20000),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        occupancy_periods: None,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_charitable_purposes: true,
            ..Default::default()
        },
        language: None,
    };

    // 1. Scots calculation
    let res_sco = engine.calculate_with_language(req.clone(), business_rates_engine::i18n::Language::Sco);
    assert!(res_sco.calculation_narrative.contains("STATUTARY CALCULATION NARRATIVE & AUDIT TRAIL"));
    assert!(res_sco.calculation_narrative.contains("1. Statutary Framewark:"));
    assert!(res_sco.calculation_narrative.contains("2. Leeabeelity Period:"));
    assert!(res_sco.calculation_narrative.contains("HALE NET AMOUNT TAE PAY:"));

    let seg_sco = &res_sco.annual_billing_segments[0];
    let charity_sco = seg_sco.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(charity_sco.relief_name, "Mandatar Chairitable Rate Relief");

    // 2. Scottish Gaelic calculation
    let res_gd = engine.calculate_with_language(req, business_rates_engine::i18n::Language::Gd);
    assert!(res_gd.calculation_narrative.contains("AITHRIS REACHDAIL AIR ÀIREAMHACHADH ÌREAN & SLIGHE SGRÙDAIDH"));
    assert!(res_gd.calculation_narrative.contains("1. Frèam Reachdail:"));
    assert!(res_gd.calculation_narrative.contains("2. Ùine Buailteachd:"));
    assert!(res_gd.calculation_narrative.contains("IOMLAN GLAN RI PHÀIGHEADH:"));

    let seg_gd = &res_gd.annual_billing_segments[0];
    let charity_gd = seg_gd.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(charity_gd.relief_name, "Faochadh Ìrean Carthannais Èigneachail");
}

#[tokio::test]
async fn test_i18n_reference_codes_localized_scots_and_gaelic() {
    let repo = ParameterRepository::new_mock();

    // Scots reference codes
    let sco_codes = repo.all_reference_codes_localized(business_rates_engine::i18n::Language::Sco);
    let england_sco = sco_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "ENGLAND").unwrap();
    assert_eq!(england_sco.label, "Ingland");
    assert!(england_sco.description.contains("Statutary non-domestic ratin"));

    let charity_sco = sco_codes.iter().find(|c| c.category == "PARTY_TYPE" && c.code == "CHARITY").unwrap();
    assert_eq!(charity_sco.label, "Registratit Chairity");

    // Gaelic reference codes
    let gd_codes = repo.all_reference_codes_localized(business_rates_engine::i18n::Language::Gd);
    let england_gd = gd_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "ENGLAND").unwrap();
    assert_eq!(england_gd.label, "Sasainn");
    assert!(england_gd.description.contains("Ìrean neo-dachaigheil reachdail"));

    let charity_gd = gd_codes.iter().find(|c| c.category == "PARTY_TYPE" && c.code == "CHARITY").unwrap();
    assert_eq!(charity_gd.label, "Carthannas Clàraichte");
}

#[tokio::test]
async fn test_i18n_localized_error_messages_scots_and_gaelic() {
    let err = business_rates_engine::error::AppError::InvalidOccupationPeriod {
        start: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
    };

    let msg_sco = err.localized_message(business_rates_engine::i18n::Language::Sco);
    assert_eq!(msg_sco, "End date for occupancy (2024-03-31) canna be afore start date (2025-04-01)");

    let msg_gd = err.localized_message(business_rates_engine::i18n::Language::Gd);
    assert_eq!(msg_gd, "Chan urrainn do cheann-latha crìochnachaidh gabhaltais (2024-03-31) a bhith ro cheann-latha tòiseachaidh (2025-04-01)");
}

#[tokio::test]
async fn test_root_ui_route_and_static_assets_with_owasp_headers() {
    let app = create_test_app();

    // 1. GET /
    let req = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
    // Strict OWASP CSP compliance
    assert!(res.headers().contains_key(header::CONTENT_SECURITY_POLICY));
    assert!(res.headers().get(header::CONTENT_SECURITY_POLICY).unwrap().to_str().unwrap().contains("default-src 'self'"));
    assert_eq!(res.headers().get(header::X_FRAME_OPTIONS).unwrap(), "DENY");

    let html = body_to_string(res.into_body()).await;
    assert!(html.contains("<div id=\"app\">"));

    // 2. Test static assets under /assets/*
    let assets_dir = format!("{}/frontend/dist/assets", env!("CARGO_MANIFEST_DIR"));
    if let Ok(entries) = std::fs::read_dir(assets_dir) {
        for entry in entries.flatten() {
            let fname = entry.file_name().to_string_lossy().to_string();
            let req_asset = Request::builder()
                .uri(format!("/assets/{}", fname))
                .method("GET")
                .body(Body::empty())
                .unwrap();
            let res_asset = app.clone().oneshot(req_asset).await.unwrap();
            assert_eq!(res_asset.status(), StatusCode::OK);
            assert!(res_asset.headers().contains_key(header::CONTENT_SECURITY_POLICY));
            if fname.ends_with(".js") {
                assert_eq!(
                    res_asset.headers().get(header::CONTENT_TYPE).unwrap(),
                    "application/javascript; charset=utf-8"
                );
            } else if fname.ends_with(".css") {
                assert_eq!(
                    res_asset.headers().get(header::CONTENT_TYPE).unwrap(),
                    "text/css; charset=utf-8"
                );
            }
        }
    }
}

#[tokio::test]
async fn test_http_parameters_endpoints() {
    let app = create_test_app();

    // 1. GET /api/v1/parameters
    let req = Request::builder()
        .uri("/api/v1/parameters")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = body_to_string(res.into_body()).await;
    let json: Value = serde_json::from_str(&body).unwrap();
    assert!(json.get("multipliers").unwrap().is_array());
    assert!(json.get("sbrr_thresholds").unwrap().is_array());
    assert!(json.get("sector_reliefs").unwrap().is_array());
    assert!(json.get("transitional_caps").unwrap().is_array());
    assert!(!json.get("multipliers").unwrap().as_array().unwrap().is_empty());

    // 2. GET /api/v1/parameters/multipliers
    let req = Request::builder()
        .uri("/api/v1/parameters/multipliers")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = body_to_string(res.into_body()).await;
    let json: Value = serde_json::from_str(&body).unwrap();
    assert!(json.as_array().unwrap().len() >= 40);

    // 3. GET /api/v1/parameters/sbrr-thresholds
    let req = Request::builder()
        .uri("/api/v1/parameters/sbrr-thresholds")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 4. GET /api/v1/parameters/sector-reliefs
    let req = Request::builder()
        .uri("/api/v1/parameters/sector-reliefs")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 5. GET /api/v1/parameters/transitional-caps
    let req = Request::builder()
        .uri("/api/v1/parameters/transitional-caps")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_openapi_and_swagger_endpoints() {
    let app = create_test_app();

    // 1. GET /api/v1/openapi.json
    let req = Request::builder()
        .uri("/api/v1/openapi.json")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "application/json; charset=utf-8"
    );
    let body = body_to_string(res.into_body()).await;
    let json: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json.get("openapi").unwrap().as_str().unwrap(), "3.1.0");
    assert!(json.get("paths").unwrap().is_object());
    assert!(json.get("components").unwrap().get("schemas").unwrap().is_object());

    // 2. Removed root /openapi.json and /openapi.yaml return 404
    for path in &["/openapi.json", "/openapi.yaml"] {
        let req = Request::builder()
            .uri(*path)
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND, "Root {} should return 404", path);
    }

    // 3. GET /api/v1/openapi.yaml
    let req = Request::builder()
        .uri("/api/v1/openapi.yaml")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/yaml; charset=utf-8"
    );
    let body_yaml = body_to_string(res.into_body()).await;
    assert!(body_yaml.contains("openapi: 3.1.0"));
    assert!(body_yaml.contains("Business Rates Statutory Calculation Engine API"));

    // 4. GET /docs (Scalar API Reference)
    let req = Request::builder()
        .uri("/docs")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
    let csp = res.headers().get(header::CONTENT_SECURITY_POLICY).unwrap().to_str().unwrap();
    assert!(csp.contains("script-src 'self';"), "script-src must be strictly 'self' with zero unsafe-inline");
    assert!(csp.contains("style-src 'self' 'unsafe-inline'"), "style-src permits unsafe-inline for dynamic theme styles");
    assert!(csp.contains("font-src 'self' data:;"), "font-src must be strictly self and data with zero external font CDNs");
    assert!(!csp.contains("cdn.jsdelivr.net"), "CSP should have zero CDN dependencies when self-hosting scalar.js");
    assert!(!csp.contains("fonts.scalar.com"), "CSP should have zero external font dependencies when using withDefaultFonts:false");
    let body_html = body_to_string(res.into_body()).await;
    assert!(body_html.contains("<title>Business Rates Statutory Engine — API Reference (Scalar)</title>"));
    assert!(body_html.contains("type=\"application/json\""), "script tag must use type=application/json to avoid inline script execution");
    assert!(body_html.contains("data-url=\"/api/v1/openapi.json\""));
    assert!(body_html.contains("\"theme\":\"solarized\""));
    assert!(body_html.contains("\"withDefaultFonts\":false"));
    assert!(body_html.contains("/assets/scalar.js"));

    // 4b. GET /docs with dynamic query parameters (templating verification)
    let req = Request::builder()
        .uri("/docs?title=Westminster%20API&theme=purple&spec_url=/custom/openapi.json")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_html = body_to_string(res.into_body()).await;
    assert!(body_html.contains("<title>Westminster API</title>"));
    assert!(body_html.contains("data-url=\"/custom/openapi.json\""));
    assert!(body_html.contains("\"theme\":\"purple\""));

    // 5. GET /assets/scalar.js (Verify local lazy-loaded asset)
    let req = Request::builder()
        .uri("/assets/scalar.js")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "application/javascript; charset=utf-8"
    );

    // 6. Removed documentation aliases return 404
    for alias in &["/scalar", "/api-reference", "/swagger", "/swagger-ui"] {
        let req = Request::builder()
            .uri(*alias)
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND, "Decommissioned alias {} should return 404", alias);
    }
}

// =========================================================================
// SUITE E: HISTORICAL STATUTORY PARAMETERS & REVALUATIONS (1993 - 2022)
// =========================================================================

#[tokio::test]
async fn test_historical_sbrr_pre_2005_returns_no_relief() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // In 2003/04 (pre-2005), SBRR did not exist in primary legislation (enacted by LGA 2003 s.61).
    let request = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2003, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2004, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2000, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(4000.00),
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

    // Multiplier for England 2003/04 is 0.444. Gross = 4000 * 0.444 = 1776.00
    assert_eq!(seg.rateable_value, dec!(4000.00));
    assert_eq!(seg.gross_liability, dec!(1776.00));
    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "SBRR"));
    assert_eq!(seg.net_segment_payable, dec!(1776.00));
}

#[tokio::test]
async fn test_historical_sbrr_2005_list_england_full_and_taper() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 2007/08 England: Small Business multiplier = 0.441
    // Full relief test: RV £4,000 (max relief in 2005 list was 50% up to £5,000 under SI 2004/3315)
    let req_full = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2007, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2008, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2005, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(4000.00),
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

    let res_full = engine.calculate(req_full);
    let seg_full = &res_full.annual_billing_segments[0];
    assert_eq!(seg_full.gross_liability, dec!(1764.00));
    let r_full = seg_full.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(r_full.discount_percentage, dec!(50.00));
    assert_eq!(r_full.amount_deducted, dec!(882.00));
    assert_eq!(seg_full.net_segment_payable, dec!(882.00));
    assert_eq!(r_full.statutory_authority, "LGFA 1988 s.43(4B) / SI 2004/3315");

    // Taper test: RV £7,500 (taper from 50% to 0% between £5,001 and £10,000)
    // Formula: (10,000 - 7,500) / 4999.99 * 50 = 25.00%
    let req_tap = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2007, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2008, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2005, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(7500.00),
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

    let res_tap = engine.calculate(req_tap);
    let seg_tap = &res_tap.annual_billing_segments[0];
    assert_eq!(seg_tap.gross_liability, dec!(3307.50));
    let r_tap = seg_tap.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(r_tap.discount_percentage, dec!(25.00));
    assert_eq!(r_tap.amount_deducted, dec!(826.88));
    assert_eq!(seg_tap.net_segment_payable, dec!(2480.62));
}

#[tokio::test]
async fn test_historical_sbrr_2010_list_england_and_scotland() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // England 2012/13: Small Business multiplier = 0.450
    // Doubled relief under SI 2010/1959: 100% up to £6,000; taper £6,001 to £12,000
    let req_eng = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2012, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2013, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(9000.00),
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

    let res_eng = engine.calculate(req_eng);
    let seg_eng = &res_eng.annual_billing_segments[0];
    // Taper at £9,000: (12,000 - 9,000) / 5999.99 * 100% = 50.00%
    let r_eng = seg_eng.itemized_reliefs.iter().find(|r| r.relief_code == "SBRR").unwrap();
    assert_eq!(r_eng.discount_percentage, dec!(50.00));
    assert_eq!(r_eng.amount_deducted, dec!(2025.00));
    assert_eq!(seg_eng.net_segment_payable, dec!(2025.00));

    // Scotland 2012/13: Multiplier = 0.450
    // SSI 2010/37: 100% <= £10,000; 50% <= £12,000; 25% <= £18,000
    let req_sco_mid = LiabilityCalculationRequest {
        party_type: PartyType::INDIVIDUAL,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2012, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2013, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::SCOTLAND,
            scat_code: "225".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(11000.00),
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

    let res_sco = engine.calculate(req_sco_mid);
    let seg_sco = &res_sco.annual_billing_segments[0];
    let r_sco = seg_sco.itemized_reliefs.iter().find(|r| r.relief_code == "SBBS").unwrap();
    assert_eq!(r_sco.discount_percentage, dec!(50.00));
    assert_eq!(r_sco.amount_deducted, dec!(2475.00));
    assert_eq!(seg_sco.net_segment_payable, dec!(2475.00));
}

#[tokio::test]
async fn test_historical_transitional_phasing_1993_94() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1993/94: Small property (RV £9,000 <= £10,000)
    // Previous year (1992/93) RV = £6,000, 1992/93 multiplier = 0.406
    // Base annual liability = 6000 * 0.406 = 2436.00
    // 1993/94 Small upward cap = +15% (factor 1.15)
    // Capped liability = 2436.00 * 1.15 = 2801.40
    // 1993/94 multiplier = 0.416. Gross = 9000 * 0.416 = 3744.00
    // Transitional relief = 3744.00 - 2801.40 = 942.60
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(1993, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1994, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(1993, 3, 31).unwrap()),
                    rateable_value: dec!(6000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1993, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(9000.00),
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
    assert_eq!(seg.gross_liability, dec!(3744.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(942.60));
    assert_eq!(seg.net_segment_payable, dec!(2801.40));
}

#[tokio::test]
async fn test_historical_transitional_phasing_1995_list_revaluation() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1995/96 Revaluation (SI 1994/3279): Large property (> £10,000 RV)
    // 1994/95 RV = £8,000, 1994/95 multiplier = 0.423. Base = 8000 * 0.423 = 3384.00
    // 1995/96 Large upward cap = +20% (factor 1.20). Capped = 3384.00 * 1.20 = 4060.80
    // 1995/96 RV = £15,000, multiplier = 0.432. Gross = 15000 * 0.432 = 6480.00
    // Transitional relief = 6480.00 - 4060.80 = 2419.20
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(1995, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1996, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(1995, 3, 31).unwrap()),
                    rateable_value: dec!(8000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1995, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(15000.00),
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
    assert_eq!(seg.gross_liability, dec!(6480.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(2419.20));
    assert_eq!(seg.net_segment_payable, dec!(4060.80));
}

#[tokio::test]
async fn test_historical_transitional_phasing_2017_list_revaluation() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 2017/18 Revaluation (SI 2016/1265): Large property (> £100,000 RV)
    // 2016/17 RV = £80,000, 2016/17 standard multiplier = 0.497. Base = 80000 * 0.497 = 39760.00
    // 2017/18 Large upward cap = +42% (factor 1.42). Capped = 39760.00 * 1.42 = 56459.20
    // 2017/18 RV = £150,000, standard multiplier = 0.479. Gross = 150000 * 0.479 = 71850.00
    // Transitional relief = 71850.00 - 56459.20 = 15390.80
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2017, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2018, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(2017, 3, 31).unwrap()),
                    rateable_value: dec!(80000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2017, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(150000.00),
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
    assert_eq!(seg.gross_liability, dec!(71850.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(15390.80));
    assert_eq!(seg.net_segment_payable, dec!(56459.20));
}

#[tokio::test]
async fn test_historical_calculation_1990_91_inaugural_nndr() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1990/91: Inaugural non-domestic rating year under LGFA 1988 (1 April 1990 to 31 March 1991)
    // England standard multiplier = 0.348
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1991, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(10000.00),
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

    // 10000 * 0.348 = 3480.00. SBRR did not exist in 1990/91.
    assert_eq!(seg.financial_year, "1990/91");
    assert_eq!(seg.rateable_value, dec!(10000.00));
    assert_eq!(seg.multiplier_used.rate, dec!(0.348));
    assert_eq!(seg.gross_liability, dec!(3480.00));
    assert!(!seg.itemized_reliefs.iter().any(|r| r.relief_code == "SBRR"));
    assert_eq!(seg.net_segment_payable, dec!(3480.00));
}

#[tokio::test]
async fn test_historical_transitional_phasing_1991_92() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1991/92: Small property (RV £8,000 <= £10,000 under SI 1989/1358)
    // Previous year (1990/91) RV = £5,000, multiplier = 0.348. Base = 5000 * 0.348 = 1740.00
    // 1991/92 Small upward cap = +15% (factor 1.15). Capped = 1740.00 * 1.15 = 2001.00
    // 1991/92 England multiplier = 0.386. Gross = 8000 * 0.386 = 3088.00
    // Transitional relief = 3088.00 - 2001.00 = 1087.00
    // Net payable = 2001.00
    let request = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(1991, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1992, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "096".to_string(),
            rateable_value_history: vec![
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1990, 4, 1).unwrap(),
                    effective_to: Some(NaiveDate::from_ymd_opt(1991, 3, 31).unwrap()),
                    rateable_value: dec!(5000.00),
                },
                RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(1991, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(8000.00),
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
    assert_eq!(seg.financial_year, "1991/92");
    assert_eq!(seg.gross_liability, dec!(3088.00));
    assert_eq!(seg.transitional_adjustment.adjustment_type, "RELIEF");
    assert_eq!(seg.transitional_adjustment.amount, dec!(1087.00));
    assert_eq!(seg.net_segment_payable, dec!(2001.00));
}





