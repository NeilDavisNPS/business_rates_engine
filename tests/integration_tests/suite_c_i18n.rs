//! Suite C: 4-Language Internationalization (i18n) & Content Negotiation
//!
//! Part of the Business Rates Calculation Engine statutory test suite.

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
async fn test_i18n_welsh_scots_and_gaelic_narrative_and_relief_names() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1. Welsh calculation
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
    let req_cy: LiabilityCalculationRequest = serde_json::from_str(json_welsh).unwrap();
    assert_eq!(req_cy.language.as_deref(), Some("cy"));
    let res_cy = engine.calculate(req_cy);

    assert!(res_cy.calculation_narrative.contains("ADRODDIAD STATUDOL AR GYFRIFO TRETHI BUSNES & LLWYBR ARCHWILIO"));
    assert!(res_cy.calculation_narrative.contains("1. Fframwaith Statudol:"));
    assert!(res_cy.calculation_narrative.contains("Deddf Cyllid Llywodraeth Leol (Cymru) 2024"));
    assert!(res_cy.calculation_narrative.contains("Blwyddyn Ariannol 2025/26:"));
    assert!(res_cy.calculation_narrative.contains("CYFANSWM Y SWM NET I'W DALU:"));
    let seg_cy = &res_cy.annual_billing_segments[0];
    let charity_cy = seg_cy.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(charity_cy.relief_name, "Rhyddhad Elusennol Gorfodol");

    // 2. Scots and Scottish Gaelic calculations
    let req_scot = LiabilityCalculationRequest {
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

    let res_sco = engine.calculate_with_language(req_scot.clone(), business_rates_engine::i18n::Language::Sco);
    assert!(res_sco.calculation_narrative.contains("STATUTARY CALCULATION NARRATIVE & AUDIT TRAIL"));
    assert!(res_sco.calculation_narrative.contains("1. Statutary Framewark:"));
    assert!(res_sco.calculation_narrative.contains("HALE NET AMOUNT TAE PAY:"));
    let seg_sco = &res_sco.annual_billing_segments[0];
    let charity_sco = seg_sco.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(charity_sco.relief_name, "Mandatar Chairitable Rate Relief");

    let res_gd = engine.calculate_with_language(req_scot, business_rates_engine::i18n::Language::Gd);
    assert!(res_gd.calculation_narrative.contains("AITHRIS REACHDAIL AIR ÀIREAMHACHADH ÌREAN & SLIGHE SGRÙDAIDH"));
    assert!(res_gd.calculation_narrative.contains("1. Frèam Reachdail:"));
    assert!(res_gd.calculation_narrative.contains("IOMLAN GLAN RI PHÀIGHEADH:"));
    let seg_gd = &res_gd.annual_billing_segments[0];
    let charity_gd = seg_gd.itemized_reliefs.iter().find(|r| r.relief_code == "CHARITY_MANDATORY").unwrap();
    assert_eq!(charity_gd.relief_name, "Faochadh Ìrean Carthannais Èigneachail");
}

#[tokio::test]
async fn test_i18n_reference_codes_localized_all_languages() {
    let repo = ParameterRepository::new_mock();

    // 1. Welsh
    let cy_codes = repo.all_reference_codes_localized(business_rates_engine::i18n::Language::Cy);
    assert_eq!(cy_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "ENGLAND").unwrap().label, "Lloegr");
    assert_eq!(cy_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "WALES").unwrap().label, "Cymru");
    assert_eq!(cy_codes.iter().find(|c| c.category == "PARTY_TYPE" && c.code == "CHARITY").unwrap().label, "Elusen Gofrestredig");
    assert_eq!(cy_codes.iter().find(|c| c.category == "OCCUPANCY_TYPE" && c.code == "OCCUPIED").unwrap().label, "Mangre a Feddienir");

    // 2. English
    let en_codes = repo.all_reference_codes_localized(business_rates_engine::i18n::Language::En);
    assert_eq!(en_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "ENGLAND").unwrap().label, "England");
    assert_eq!(en_codes.iter().find(|c| c.category == "PARTY_TYPE" && c.code == "CHARITY").unwrap().label, "Registered Charity");

    // 3. Scots
    let sco_codes = repo.all_reference_codes_localized(business_rates_engine::i18n::Language::Sco);
    assert_eq!(sco_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "ENGLAND").unwrap().label, "Ingland");
    assert_eq!(sco_codes.iter().find(|c| c.category == "PARTY_TYPE" && c.code == "CHARITY").unwrap().label, "Registratit Chairity");

    // 4. Gaelic
    let gd_codes = repo.all_reference_codes_localized(business_rates_engine::i18n::Language::Gd);
    assert_eq!(gd_codes.iter().find(|c| c.category == "JURISDICTION" && c.code == "ENGLAND").unwrap().label, "Sasainn");
    assert_eq!(gd_codes.iter().find(|c| c.category == "PARTY_TYPE" && c.code == "CHARITY").unwrap().label, "Carthannas Clàraichte");
}

#[tokio::test]
async fn test_i18n_localized_error_messages_all_languages() {
    let err = business_rates_engine::error::AppError::InvalidOccupationPeriod {
        start: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
    };

    assert_eq!(err.localized_message(business_rates_engine::i18n::Language::En), "Occupation end date (2024-03-31) cannot precede start date (2025-04-01)");
    assert_eq!(err.localized_message(business_rates_engine::i18n::Language::Cy), "Ni all dyddiad gorffen meddiannaeth (2024-03-31) fod cyn y dyddiad cychwyn (2025-04-01)");
    assert_eq!(err.localized_message(business_rates_engine::i18n::Language::Sco), "End date for occupancy (2024-03-31) canna be afore start date (2025-04-01)");
    assert_eq!(err.localized_message(business_rates_engine::i18n::Language::Gd), "Chan urrainn do cheann-latha crìochnachaidh gabhaltais (2024-03-31) a bhith ro cheann-latha tòiseachaidh (2025-04-01)");
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

    // 5. q=0 excludes a language, and English preferred over Welsh stays English
    for (al, expected) in [
        ("en-GB,en;q=0.9,cy;q=0", Language::En),
        ("en-GB,cy;q=0.5", Language::En),
        ("cy;q=0.5,gd", Language::Gd),
        ("fr,sco;q=0.4,cy;q=0.4", Language::Sco),
        ("gd-GB;q=0.8,cy-GB;q=0.9", Language::Cy),
        // Only the primary subtag counts: "fr-cy" is French; a malformed q-value drops the tag
        ("fr-cy,de", Language::En),
        ("cy;q=abc", Language::En),
    ] {
        let mut h = HeaderMap::new();
        h.insert(header::ACCEPT_LANGUAGE, al.parse().unwrap());
        assert_eq!(resolve_language(&h, None, None), expected, "Accept-Language: {al}");
    }
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

#[test]
fn test_domain_error_status_codes_and_codes() {
    let err_period = AppError::InvalidOccupationPeriod {
        start: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
    };
    assert_eq!(err_period.status_code(), StatusCode::BAD_REQUEST);
    assert_eq!(err_period.error_code(), "INVALID_OCCUPATION_PERIOD");

    let err_overlap = AppError::OverlappingOccupationPeriods {
        first_start: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
        first_end: NaiveDate::from_ymd_opt(2024, 8, 31).unwrap(),
        second_start: NaiveDate::from_ymd_opt(2024, 8, 1).unwrap(),
        second_end: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };
    assert_eq!(err_overlap.status_code(), StatusCode::BAD_REQUEST);
    assert_eq!(err_overlap.error_code(), "OVERLAPPING_OCCUPATION_PERIODS");

    let err_rv_missing = AppError::RateableValueMissingForPeriod {
        period_start: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
        rv_start: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
    };
    assert_eq!(err_rv_missing.status_code(), StatusCode::BAD_REQUEST);
    assert_eq!(err_rv_missing.error_code(), "RATEABLE_VALUE_MISSING_FOR_PERIOD");

    let err_payload = AppError::PayloadDeserialization("invalid json syntax".to_string());
    assert_eq!(err_payload.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(err_payload.error_code(), "PAYLOAD_DESERIALIZATION_ERROR");
}

// =========================================================================
// SUITE E: OWASP RECOMMENDED SECURE HEADERS
// =========================================================================

#[tokio::test]
async fn test_narrative_relief_assessment_audit_unawarded_reliefs() {
    let repo = ParameterRepository::new_seeded();
    let engine = CalculationEngine::new(repo);

    // Case 1: Commercial office (SCAT 091) RV £45,000 claiming Rural Settlement Relief in England
    let req_rural_fail = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(45000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rural_settlement: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: Some("en".to_string()),
    };
    let res_rural_fail = engine.calculate(req_rural_fail);
    let nar = &res_rural_fail.calculation_narrative;
    assert!(nar.contains("Relief Assessment Audit (Unawarded Reliefs):"));
    assert!(nar.contains("Rural Settlement Relief: Disqualified — Rateable Value (£45000.00) exceeds statutory cap of £8,500 for commercial/general hereditaments [LGFA 1988 s.43(6B) / LGFA 1997]"));

    // Case 2: Welsh language narrative audit for the same disqualification
    let req_rural_cy = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(45000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rural_settlement: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: Some("cy".to_string()),
    };
    let res_rural_cy = engine.calculate(req_rural_cy);
    let nar_cy = &res_rural_cy.calculation_narrative;
    assert!(nar_cy.contains("Archwiliad Asesu Rhyddhadau (Rhyddhadau heb eu Dyfarnu):"));
    assert!(nar_cy.contains("Rhyddhad Anheddiad Gwledig: Wedi'i anghymhwyso — mae'r Gwerth Trethiannol (£45000.00) yn fwy na'r cap statudol o £8,500 ar gyfer hereditamentau masnachol/cyffredinol [LGFA 1988 s.43(6B) / LGFA 1997]"));

    // Case 3: SBRR claimed with RV £25,000 (> £15,000 cap in England)
    let req_sbrr_fail = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(25000.00),
            }],
        },
        occupancy_type: OccupancyType::OCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_small_business_sole_property: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: Some("en".to_string()),
    };
    let res_sbrr_fail = engine.calculate(req_sbrr_fail);
    let nar_sbrr = &res_sbrr_fail.calculation_narrative;
    assert!(nar_sbrr.contains("Small Business Rate Relief: Disqualified — Rateable Value (£25000.00) exceeds £15,000 statutory taper ceiling [LGFA 1988 s.43(4B) / S.I. 2017/411]"));

    // Case 4: Unoccupied property claiming occupied relief
    let req_unocc_fail = LiabilityCalculationRequest {
        party_type: PartyType::CORPORATE,
        occupation_period: DateRange {
            start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        },
        hereditament: HereditamentInput {
            jurisdiction: Jurisdiction::ENGLAND,
            scat_code: "091".to_string(),
            rateable_value_history: vec![RvEntry {
                effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                effective_to: None,
                rateable_value: dec!(5000.00),
            }],
        },
        occupancy_type: OccupancyType::UNOCCUPIED,
        qualifying_relief_flags: QualifyingReliefFlags {
            is_rural_settlement: true,
            ..Default::default()
        },
        occupancy_periods: None,
        language: Some("en".to_string()),
    };
    let res_unocc_fail = engine.calculate(req_unocc_fail);
    let nar_unocc = &res_unocc_fail.calculation_narrative;
    assert!(nar_unocc.contains("Rural Settlement Relief: Disqualified — property is unoccupied (mandatory rural settlement relief requires qualifying occupation)"));
}

#[tokio::test]
async fn test_narrative_decision_traceability_across_languages() {
    let repo = ParameterRepository::new_mock();
    let engine = CalculationEngine::new(repo);

    // 1. SBRR in taper band (RV £13,500 in England) with 10 monthly instalments
    for (lang, expected_mult, expected_taper, expected_inst) in [
        ("en", "Multiplier Determination: Qualified for Small Business Multiplier", "SBRR Taper Evaluation: Assessed using statutory divisor E", "Statutory Payment Schedule: 10 monthly instalments"),
        ("cy", "Penderfyniad ar Luosydd: Yn gymwys ar gyfer Lluosydd Busnesau Bach", "Gwerthusiad Taper SBRR: Aseswyd gan ddefnyddio rhannydd statudol E", "Atodlen Daliadau Statudol: 10 rhandaliad misol"),
        ("sco", "Multiplier Determination: Qualified fer Sma Business Multiplier", "SBRR Taper Evaluation: Assessed uisin statutary divisor E", "Statutary Payment Schedule: 10 monthly instalments"),
        ("gd", "Co-dhùnadh Iomadair: Teisteanasach airson Iomadair Gnìomhachasan Beaga", "Measadh Lùghdachadh SBRR: Air a mheasadh a' cleachdadh roinniche reachdail E", "Clàr-ama Pàighidh Reachdail: 10 cuibhreannan mìosail"),
    ] {
        let req = LiabilityCalculationRequest {
            party_type: PartyType::INDIVIDUAL,
            occupation_period: DateRange {
                start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
            },
            hereditament: HereditamentInput {
                jurisdiction: Jurisdiction::ENGLAND,
                scat_code: "091".to_string(),
                rateable_value_history: vec![RvEntry {
                    effective_from: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
                    effective_to: None,
                    rateable_value: dec!(13500.00),
                }],
            },
            occupancy_type: OccupancyType::OCCUPIED,
            qualifying_relief_flags: QualifyingReliefFlags {
                is_small_business_sole_property: true,
                payment_installments_count: Some(10),
                ..Default::default()
            },
            occupancy_periods: None,
            language: Some(lang.to_string()),
        };

        let res = engine.calculate(req);
        let narrative = &res.calculation_narrative;
        assert!(narrative.contains(expected_mult), "Language {} missing multiplier justification: {}", lang, narrative);
        assert!(narrative.contains(expected_taper), "Language {} missing taper divisor note: {}", lang, narrative);
        assert!(narrative.contains(expected_inst), "Language {} missing installment schedule note: {}", lang, narrative);
    }
}

