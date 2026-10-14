//! Suite I: Component UI Serving & OWASP Security
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
    assert!(html.contains("<title>Business Rates Calculation Engine — Statutory Calculation Sandbox</title>"));
    assert!(html.contains(r#"<link rel="icon" type="image/svg+xml""#));
    assert!((html.contains("/assets/app.js") || html.contains("/assets/index-")) && html.contains(".css"));
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

#[tokio::test]
async fn test_static_assets_etag_cache_control_and_version_busting() {
    let app = create_test_app();

    // 1. Root / UI serves index.html with no-cache and versioned script/css tags
    let req_root = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_root = app.clone().oneshot(req_root).await.unwrap();
    assert_eq!(res_root.status(), StatusCode::OK);
    assert_eq!(
        res_root.headers().get(header::CACHE_CONTROL).unwrap(),
        "no-cache, must-revalidate"
    );
    let html_root = body_to_string(res_root.into_body()).await;
    assert!(html_root.contains("/assets/app.js?v="));
    assert!(html_root.contains("/assets/app.css?v="));

    // 2. /docs serves scalar.html with no-cache and versioned scalar.js tag
    let req_docs = Request::builder()
        .uri("/docs")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_docs = app.clone().oneshot(req_docs).await.unwrap();
    assert_eq!(res_docs.status(), StatusCode::OK);
    assert_eq!(
        res_docs.headers().get(header::CACHE_CONTROL).unwrap(),
        "no-cache, must-revalidate"
    );
    let html_docs = body_to_string(res_docs.into_body()).await;
    assert!(html_docs.contains("/assets/scalar.js?v="));

    // 3. /assets/app.js with ?v= query returns immutable cache header and ETag
    let req_app_js = Request::builder()
        .uri("/assets/app.js?v=0.0.1-test")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_app_js = app.clone().oneshot(req_app_js).await.unwrap();
    assert_eq!(res_app_js.status(), StatusCode::OK);
    assert_eq!(
        res_app_js.headers().get(header::CACHE_CONTROL).unwrap(),
        "public, max-age=31536000, immutable"
    );
    let app_js_etag = res_app_js.headers().get(header::ETAG).unwrap().to_str().unwrap().to_string();
    assert!(app_js_etag.starts_with('"') && app_js_etag.ends_with('"'));

    // 4. /assets/app.js with matching If-None-Match returns 304 NOT MODIFIED
    let req_app_js_304 = Request::builder()
        .uri("/assets/app.js?v=0.0.1-test")
        .method("GET")
        .header(header::IF_NONE_MATCH, &app_js_etag)
        .body(Body::empty())
        .unwrap();
    let res_app_js_304 = app.clone().oneshot(req_app_js_304).await.unwrap();
    assert_eq!(res_app_js_304.status(), StatusCode::NOT_MODIFIED);
    assert_eq!(res_app_js_304.headers().get(header::ETAG).unwrap(), &app_js_etag);
    let body_304 = body_to_string(res_app_js_304.into_body()).await;
    assert!(body_304.is_empty(), "304 response body must be empty");

    // 5. /assets/scalar.js with ?v= query returns immutable cache header and ETag
    let req_scalar = Request::builder()
        .uri("/assets/scalar.js?v=0.0.1-test")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res_scalar = app.clone().oneshot(req_scalar).await.unwrap();
    assert_eq!(res_scalar.status(), StatusCode::OK);
    assert_eq!(
        res_scalar.headers().get(header::CACHE_CONTROL).unwrap(),
        "public, max-age=31536000, immutable"
    );
    let scalar_etag = res_scalar.headers().get(header::ETAG).unwrap().to_str().unwrap().to_string();

    // 6. /assets/scalar.js with matching If-None-Match returns 304 NOT MODIFIED
    let req_scalar_304 = Request::builder()
        .uri("/assets/scalar.js")
        .method("GET")
        .header(header::IF_NONE_MATCH, &scalar_etag)
        .body(Body::empty())
        .unwrap();
    let res_scalar_304 = app.oneshot(req_scalar_304).await.unwrap();
    assert_eq!(res_scalar_304.status(), StatusCode::NOT_MODIFIED);
    let scalar_body_304 = body_to_string(res_scalar_304.into_body()).await;
    assert!(scalar_body_304.is_empty(), "304 response body must be empty for scalar.js");
}

