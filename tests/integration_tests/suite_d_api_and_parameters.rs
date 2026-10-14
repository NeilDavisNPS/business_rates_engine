//! Suite D: Reference Codes, Parameter Resolution, Public API Access & OpenAPI
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
async fn test_app_state_repository_access() {
    let state = AppState {
        repo: ParameterRepository::new_mock(),
    };
    assert!(!state.repo.all_reference_codes().is_empty());
}

#[tokio::test]
async fn test_zero_hardcoding_historical_multipliers_1990_to_2026() {
    let repo = ParameterRepository::new_seeded();

    // 1990/91 Statutory Multipliers (Inaugural NNDR under LGFA 1988)
    let m_90_eng = repo.resolve_multiplier(Jurisdiction::ENGLAND, "1990/91", dec!(25000.00)).unwrap();
    assert_eq!(m_90_eng.rate, dec!(0.348));

    let m_90_wal = repo.resolve_multiplier(Jurisdiction::WALES, "1990/91", dec!(25000.00)).unwrap();
    assert_eq!(m_90_wal.rate, dec!(0.368));

    let m_90_sco = repo.resolve_multiplier(Jurisdiction::SCOTLAND, "1990/91", dec!(25000.00)).unwrap();
    assert_eq!(m_90_sco.rate, dec!(0.410));

    // 1993/94 Statutory Multipliers
    let m_93_eng = repo.resolve_multiplier(Jurisdiction::ENGLAND, "1993/94", dec!(25000.00)).unwrap();
    assert_eq!(m_93_eng.rate, dec!(0.416));

    let m_93_sco = repo.resolve_multiplier(Jurisdiction::SCOTLAND, "1993/94", dec!(25000.00)).unwrap();
    assert_eq!(m_93_sco.rate, dec!(0.477));

    // 2000/01 Revaluation List
    let m_00_eng = repo.resolve_multiplier(Jurisdiction::ENGLAND, "2000/01", dec!(10000.00)).unwrap();
    assert_eq!(m_00_eng.rate, dec!(0.416));

    // 2010/11 Revaluation List (Small Business vs Standard)
    let m_10_small = repo.resolve_multiplier(Jurisdiction::ENGLAND, "2010/11", dec!(12000.00)).unwrap();
    assert_eq!(m_10_small.rate, dec!(0.407));
    assert_eq!(m_10_small.multiplier_type, "SMALL_BUSINESS");

    let m_10_std = repo.resolve_multiplier(Jurisdiction::ENGLAND, "2010/11", dec!(60000.00)).unwrap();
    assert_eq!(m_10_std.rate, dec!(0.414));
    assert_eq!(m_10_std.multiplier_type, "STANDARD");

    let m_10_wal = repo.resolve_multiplier(Jurisdiction::WALES, "2010/11", dec!(25000.00)).unwrap();
    assert_eq!(m_10_wal.rate, dec!(0.409));

    let m_16_wal = repo.resolve_multiplier(Jurisdiction::WALES, "2016/17", dec!(25000.00)).unwrap();
    assert_eq!(m_16_wal.rate, dec!(0.486));

    // 2026/27 High-Value Threshold (£500k)
    let m_26_hv = repo.resolve_multiplier(Jurisdiction::ENGLAND, "2026/27", dec!(500000.00)).unwrap();
    assert_eq!(m_26_hv.rate, dec!(0.508));
    assert_eq!(m_26_hv.multiplier_type, "HIGH_VALUE");
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
    assert!(reliefs.iter().any(|c| c.code == "RHL"));
    assert!(reliefs.iter().any(|c| c.code == "WELSH_AVOIDANCE" && c.jurisdiction.as_deref() == Some("WALES")));
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

    // 4c. GET /docs escapes title and spec_url, and rejects unknown theme/layout and off-site spec URLs
    let req = Request::builder()
        .uri("/docs?title=%3C/title%3E%3Cmeta%20http-equiv%3Drefresh%3E&theme=x%22%7D%27%3E%3Cb%3E&layout=%3Ci%3E&spec_url=//evil.example/spec.json")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_html = body_to_string(res.into_body()).await;
    assert!(body_html.contains("<title>&lt;/title&gt;&lt;meta http-equiv=refresh&gt;</title>"));
    assert!(!body_html.contains("<meta http-equiv"));
    assert!(!body_html.contains("<b>") && !body_html.contains("<i>"));
    assert!(body_html.contains("\"theme\":\"solarized\""));
    assert!(body_html.contains("\"layout\":\"modern\""));
    assert!(body_html.contains("data-url=\"/api/v1/openapi.json\""));
    assert!(!body_html.contains("evil.example"));

    let req = Request::builder()
        .uri("/docs?spec_url=/spec.json%22%20onload%3D%22x")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    let body_html = body_to_string(res.into_body()).await;
    assert!(body_html.contains("data-url=\"/spec.json&quot; onload=&quot;x\""));

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
async fn test_licenses_endpoints() {
    let app = create_test_app();

    // 1. GET /licenses (HTML format)
    let res_html = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/licenses")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_html.status(), StatusCode::OK);
    let content_type = res_html.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(content_type.contains("text/html"));

    let body_html = body_to_string(res_html.into_body()).await;
    assert!(body_html.contains("<title>Third-Party Software Licenses — Business Rates Engine</title>"));
    assert!(body_html.contains("The MIT License"));
    assert!(body_html.contains("The Apache License, Version 2.0"));
    assert!(!body_html.contains("Overview & License Summary"));

    // 2. GET /licenses.md (Markdown format)
    let res_md = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/licenses.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_md.status(), StatusCode::OK);
    let md_content_type = res_md.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(md_content_type.contains("text/markdown"));

    let body_md = body_to_string(res_md.into_body()).await;
    assert!(body_md.contains("# Third-Party Software Licenses\n"));
    assert!(body_md.contains("## 1. Rust Backend Dependencies"));
    assert!(body_md.contains("## 2. Frontend Dependencies"));
    assert!(!body_md.contains("Overview & License Summary"));

    // 3. GET /api/v1/licenses?format=md (API endpoint with format parameter)
    let res_api_md = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/licenses?format=md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_api_md.status(), StatusCode::OK);
    let body_api_md = body_to_string(res_api_md.into_body()).await;
    assert!(body_api_md.contains("# Third-Party Software Licenses\n"));
}

#[tokio::test]
async fn test_health_check_endpoints() {
    let app = create_test_app();

    // 1. Root /health
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let headers = response.headers().clone();
    assert!(headers.get("x-content-type-options").is_some());
    assert!(headers.get("x-frame-options").is_some());

    let body_str = body_to_string(response.into_body()).await;
    let json: Value = serde_json::from_str(&body_str).unwrap();

    assert_eq!(json["status"], "pass");
    assert_eq!(json["checks"]["engine_api"], "pass");
    assert_eq!(json["checks"]["parameters_repository"], "pass");
    assert!(json["version"].is_string());

    // 2. API v1 /api/v1/health
    let v1_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(v1_response.status(), StatusCode::OK);
    let v1_body_str = body_to_string(v1_response.into_body()).await;
    let v1_json: Value = serde_json::from_str(&v1_body_str).unwrap();

    assert_eq!(v1_json["status"], "pass");
    assert_eq!(v1_json["checks"]["parameters_repository"], "pass");
}
