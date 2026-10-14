//! Suite I: Component UI Serving & OWASP Security
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
async fn test_owasp_secure_headers_on_static_routes() {
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

    // 2. Missing CSS asset must return 404 NOT FOUND, not fallback to an arbitrary stylesheet
    let req = Request::builder()
        .uri("/assets/missing-or-stale-style.css")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "Missing CSS asset must return 404");
}
