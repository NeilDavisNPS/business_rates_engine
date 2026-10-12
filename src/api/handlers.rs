use axum::{
    extract::{rejection::JsonRejection, Query, State},
    http::HeaderMap,
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::models::AppState;
use crate::models::calculation::{LiabilityCalculationRequest, LiabilityCalculationResponse};
use crate::engine::calculator::CalculationEngine;
use crate::db::repository::{
    ParamMultiplier, ParamReferenceCode, ParamSbrrSbbsThreshold, ParamSectorRelief,
    ParamTransitionalCap,
};
use crate::error::AppError;
use crate::i18n::Language;

/// Optional query parameters for language selection
#[derive(Debug, Deserialize, Default)]
pub struct LangQuery {
    pub lang: Option<String>,
}

/// Resolves client's preferred language from query param, payload field, or Accept-Language header
pub fn resolve_language(
    headers: &HeaderMap,
    query_lang: Option<&str>,
    payload_lang: Option<&str>,
) -> Language {
    if let Some(l) = payload_lang {
        return Language::from_code(l);
    }
    if let Some(l) = query_lang {
        return Language::from_code(l);
    }
    if let Some(al) = headers.get("Accept-Language").and_then(|v| v.to_str().ok()) {
        let low = al.to_lowercase();
        if low.contains("cy") {
            return Language::Cy;
        } else if low.contains("sco") {
            return Language::Sco;
        } else if low.contains("gd") || low.contains("gla") {
            return Language::Gd;
        }
    }
    Language::En
}

/// Statutory liability calculation API endpoint (Unauthenticated Data Plane).
/// Errors and narratives are localized based on query parameter (?lang=cy), payload field ("language": "cy"), or Accept-Language header.
pub async fn calculate_liability_handler(
    State(state): State<AppState>,
    Query(query): Query<LangQuery>,
    headers: HeaderMap,
    payload_res: Result<Json<LiabilityCalculationRequest>, JsonRejection>,
) -> Result<Json<LiabilityCalculationResponse>, Response> {
    let initial_lang = resolve_language(&headers, query.lang.as_deref(), None);

    let Json(payload) = match payload_res {
        Ok(p) => p,
        Err(rejection) => {
            let err = AppError::PayloadDeserialization(rejection.body_text());
            return Err(err.into_response_with_lang(initial_lang));
        }
    };

    let effective_lang = resolve_language(&headers, query.lang.as_deref(), payload.language.as_deref());

    for period in payload.resolve_occupancy_periods() {
        if period.end_date < period.start_date {
            let err = AppError::InvalidOccupationPeriod {
                start: period.start_date,
                end: period.end_date,
            };
            return Err(err.into_response_with_lang(effective_lang));
        }
    }

    let engine = CalculationEngine::new(state.repo.clone());
    Ok(Json(engine.calculate_with_language(payload, effective_lang)))
}

/// Returns all pre-seeded reference codes from the database catalog localized in English, Welsh, Scots, or Gaelic.
pub async fn get_reference_codes_handler(
    State(state): State<AppState>,
    Query(query): Query<LangQuery>,
    headers: HeaderMap,
) -> Json<Vec<ParamReferenceCode>> {
    let lang = resolve_language(&headers, query.lang.as_deref(), None);
    Json(state.repo.all_reference_codes_localized(lang))
}

#[derive(Debug, Clone, Serialize)]
pub struct AllParametersResponse {
    pub multipliers: Vec<ParamMultiplier>,
    pub sbrr_thresholds: Vec<ParamSbrrSbbsThreshold>,
    pub sector_reliefs: Vec<ParamSectorRelief>,
    pub transitional_caps: Vec<ParamTransitionalCap>,
}

/// Returns all statutory parameters from the in-memory parameter catalog.
pub async fn get_all_parameters_handler(
    State(state): State<AppState>,
) -> Json<AllParametersResponse> {
    Json(AllParametersResponse {
        multipliers: state.repo.all_multipliers().to_vec(),
        sbrr_thresholds: state.repo.all_sbrr_thresholds().to_vec(),
        sector_reliefs: state.repo.all_sector_reliefs().to_vec(),
        transitional_caps: state.repo.all_transitional_caps().to_vec(),
    })
}

/// Returns all statutory rating multipliers (1990–2027)
pub async fn get_multipliers_handler(
    State(state): State<AppState>,
) -> Json<Vec<ParamMultiplier>> {
    Json(state.repo.all_multipliers().to_vec())
}

/// Returns all Small Business Rate Relief / SBBS thresholds and tapers
pub async fn get_sbrr_thresholds_handler(
    State(state): State<AppState>,
) -> Json<Vec<ParamSbrrSbbsThreshold>> {
    Json(state.repo.all_sbrr_thresholds().to_vec())
}

/// Returns all devolved sector relief schemes (Retail, Hospitality & Leisure)
pub async fn get_sector_reliefs_handler(
    State(state): State<AppState>,
) -> Json<Vec<ParamSectorRelief>> {
    Json(state.repo.all_sector_reliefs().to_vec())
}

/// Returns all transitional phasing relief caps (2017 & 2023 lists)
pub async fn get_transitional_caps_handler(
    State(state): State<AppState>,
) -> Json<Vec<ParamTransitionalCap>> {
    Json(state.repo.all_transitional_caps().to_vec())
}

/// Serves the modern component-based frontend (Svelte 5 + Vite build)
pub async fn component_ui_handler() -> ([(axum::http::HeaderName, &'static str); 1], String) {
    let p = format!("{}/frontend/dist/index.html", env!("CARGO_MANIFEST_DIR"));
    let html = std::fs::read_to_string(&p)
        .or_else(|_| std::fs::read_to_string("frontend/dist/index.html"))
        .unwrap_or_else(|_| "<!DOCTYPE html><html><body><div id=\"app\">Frontend dist not found. Please run ./build-frontend.sh</div></body></html>".to_string());
    (
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    )
}

/// Serves static assets generated by the Vite build under /assets/*
pub async fn static_frontend_asset(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> Result<([(axum::http::HeaderName, &'static str); 1], Vec<u8>), (axum::http::StatusCode, &'static str)> {
    let clean_name = std::path::Path::new(&filename)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or((axum::http::StatusCode::BAD_REQUEST, "Invalid path"))?;

    let p = format!("{}/frontend/dist/assets/{}", env!("CARGO_MANIFEST_DIR"), clean_name);
    let bytes = std::fs::read(&p)
        .or_else(|_| std::fs::read(format!("frontend/dist/assets/{}", clean_name)))
        .map_err(|_| (axum::http::StatusCode::NOT_FOUND, "Asset not found"))?;

    let content_type = if clean_name.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if clean_name.ends_with(".css") {
        "text/css; charset=utf-8"
    } else {
        "application/octet-stream"
    };

    Ok((
        [(axum::http::header::CONTENT_TYPE, content_type)],
        bytes,
    ))
}

#[derive(Debug, Deserialize, Default)]
pub struct DocQueryParams {
    pub title: Option<String>,
    pub spec_url: Option<String>,
    pub theme: Option<String>,
    pub layout: Option<String>,
}

pub static OPENAPI_JSON: &str = include_str!("../../docs/openapi.json");
pub static OPENAPI_YAML: &str = include_str!("../../docs/openapi.yaml");
pub static SCALAR_TEMPLATE: &str = include_str!("../../templates/scalar.html");

/// Serves raw OpenAPI 3.1 JSON definition
pub async fn openapi_json_handler() -> ([(axum::http::HeaderName, &'static str); 1], &'static str) {
    (
        [(axum::http::header::CONTENT_TYPE, "application/json; charset=utf-8")],
        OPENAPI_JSON,
    )
}

/// Serves raw OpenAPI 3.1 YAML definition
pub async fn openapi_yaml_handler() -> ([(axum::http::HeaderName, &'static str); 1], &'static str) {
    (
        [(axum::http::header::CONTENT_TYPE, "text/yaml; charset=utf-8")],
        OPENAPI_YAML,
    )
}

/// Serves interactive Scalar API reference interface rendered from templates/scalar.html
pub async fn scalar_ui_handler(
    Query(params): Query<DocQueryParams>,
) -> ([(axum::http::HeaderName, &'static str); 1], String) {
    let title = params.title.as_deref().unwrap_or("Business Rates Statutory Engine — API Reference (Scalar)");
    let spec_url = params.spec_url.as_deref().unwrap_or("/api/v1/openapi.json");
    let theme = params.theme.as_deref().unwrap_or("solarized");
    let layout = params.layout.as_deref().unwrap_or("modern");

    let rendered = SCALAR_TEMPLATE
        .replace("{{TITLE}}", title)
        .replace("{{SPEC_URL}}", spec_url)
        .replace("{{THEME}}", theme)
        .replace("{{LAYOUT}}", layout)
        .replace("{{CUSTOM_CSS}}", "");

    (
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        rendered,
    )
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubsystemChecks {
    pub engine_api: &'static str,
    pub parameters_repository: &'static str,
    pub frontend_bundle: &'static str,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthCheckResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub checks: SubsystemChecks,
}

/// Standards-compliant RFC-style health check endpoint verifying engine API, repository, and frontend assets
pub async fn health_check_handler(
    State(state): State<AppState>,
) -> (axum::http::StatusCode, Json<HealthCheckResponse>) {
    let repo_healthy = !state.repo.all_multipliers().is_empty();

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let primary_dist = format!("{}/frontend/dist/index.html", manifest_dir);
    let frontend_healthy = std::path::Path::new(&primary_dist).exists()
        || std::path::Path::new("frontend/dist/index.html").exists()
        || std::path::Path::new("/app/frontend/dist/index.html").exists();

    let all_healthy = repo_healthy;
    let status = if all_healthy && frontend_healthy {
        "pass"
    } else if all_healthy {
        "warn"
    } else {
        "fail"
    };

    let status_code = if all_healthy {
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        Json(HealthCheckResponse {
            status,
            version: env!("CARGO_PKG_VERSION"),
            checks: SubsystemChecks {
                engine_api: "pass",
                parameters_repository: if repo_healthy { "pass" } else { "fail" },
                frontend_bundle: if frontend_healthy { "pass" } else { "warn" },
            },
        }),
    )
}


