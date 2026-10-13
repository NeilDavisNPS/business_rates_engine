use axum::{
    extract::{rejection::JsonRejection, Query, State},
    http::HeaderMap,
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::models::AppState;
use crate::models::calculation::LiabilityCalculationRequest;
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
        if let Some(lang) = parse_accept_language(al) {
            return lang;
        }
    }
    Language::En
}

/// Picks the most preferred supported language from an Accept-Language header (RFC 9110 §12.5.4).
/// Tags are matched on their primary subtag, `q=0` excludes a tag, and ties keep header order.
fn parse_accept_language(header: &str) -> Option<Language> {
    let mut ranges: Vec<(String, f32)> = header
        .split(',')
        .filter_map(|item| {
            let mut parts = item.split(';');
            let tag = parts.next()?.trim().to_ascii_lowercase();
            if tag.is_empty() {
                return None;
            }
            let mut q = 1.0_f32;
            for param in parts {
                if let Some((k, v)) = param.split_once('=') {
                    if k.trim().eq_ignore_ascii_case("q") {
                        q = v.trim().parse().ok().filter(|q: &f32| (0.0..=1.0).contains(q))?;
                    }
                }
            }
            (q > 0.0).then_some((tag, q))
        })
        .collect();
    // Stable sort keeps header order for equal q-values
    ranges.sort_by(|a, b| b.1.total_cmp(&a.1));

    ranges.iter().find_map(|(tag, _)| {
        match tag.split('-').next().unwrap_or_default() {
            "en" => Some(Language::En),
            "cy" => Some(Language::Cy),
            "sco" => Some(Language::Sco),
            "gd" | "gla" => Some(Language::Gd),
            _ => None,
        }
    })
}

/// Statutory liability calculation API endpoint (Unauthenticated Data Plane).
/// Errors and narratives are localized based on query parameter (?lang=cy), payload field ("language": "cy"), or Accept-Language header.
pub async fn calculate_liability_handler(
    State(state): State<AppState>,
    Query(query): Query<LangQuery>,
    headers: HeaderMap,
    payload_res: Result<Json<LiabilityCalculationRequest>, JsonRejection>,
) -> Response {
    let initial_lang = resolve_language(&headers, query.lang.as_deref(), None);

    let Json(payload) = match payload_res {
        Ok(p) => p,
        Err(rejection) => {
            let err = AppError::PayloadDeserialization(rejection.body_text());
            return err.into_response_with_lang(initial_lang);
        }
    };

    let effective_lang = resolve_language(&headers, query.lang.as_deref(), payload.language.as_deref());

    let mut periods = payload.resolve_occupancy_periods();
    for period in &periods {
        if period.end_date < period.start_date {
            let err = AppError::InvalidOccupationPeriod {
                start: period.start_date,
                end: period.end_date,
            };
            return err.into_response_with_lang(effective_lang);
        }
    }

    // End dates are inclusive, so a period must start after the previous one ends
    periods.sort_by_key(|p| (p.start_date, p.end_date));
    for pair in periods.windows(2) {
        if pair[1].start_date <= pair[0].end_date {
            let err = AppError::OverlappingOccupationPeriods {
                first_start: pair[0].start_date,
                first_end: pair[0].end_date,
                second_start: pair[1].start_date,
                second_end: pair[1].end_date,
            };
            return err.into_response_with_lang(effective_lang);
        }
    }

    let earliest_rv = payload.hereditament.rateable_value_history.iter().map(|e| e.effective_from).min();
    for period in &periods {
        match earliest_rv {
            Some(earliest) if period.start_date < earliest => {
                let err = AppError::RateableValueMissingForPeriod {
                    period_start: period.start_date,
                    rv_start: earliest,
                };
                return err.into_response_with_lang(effective_lang);
            }
            None => {
                let err = AppError::RateableValueMissingForPeriod {
                    period_start: period.start_date,
                    rv_start: period.start_date,
                };
                return err.into_response_with_lang(effective_lang);
            }
            _ => {}
        }
    }

    let engine = CalculationEngine::new(state.repo.clone());
    axum::response::IntoResponse::into_response(Json(engine.calculate_with_language(payload, effective_lang)))
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
    let html = match tokio::fs::read_to_string(&p).await {
        Ok(h) => h,
        Err(_) => tokio::fs::read_to_string("frontend/dist/index.html")
            .await
            .unwrap_or_else(|_| "<!DOCTYPE html><html><body><div id=\"app\">Frontend dist not found. Please run ./build-frontend.sh</div></body></html>".to_string()),
    };
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
    let bytes = match tokio::fs::read(&p).await {
        Ok(b) => b,
        Err(_) => tokio::fs::read(format!("frontend/dist/assets/{}", clean_name))
            .await
            .map_err(|_| (axum::http::StatusCode::NOT_FOUND, "Asset not found"))?,
    };

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

const SCALAR_THEMES: &[&str] = &[
    "default", "alternate", "moon", "purple", "solarized", "bluePlanet", "saturn",
    "kepler", "mars", "deepSpace", "laserwave", "elysiajs", "fastify", "none",
];
const SCALAR_LAYOUTS: &[&str] = &["modern", "classic"];

/// Escapes text for use in HTML element content and quoted attribute values
fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Serves interactive Scalar API reference interface rendered from templates/scalar.html
pub async fn scalar_ui_handler(
    Query(params): Query<DocQueryParams>,
) -> ([(axum::http::HeaderName, &'static str); 1], String) {
    let title = params.title.as_deref().unwrap_or("Business Rates Statutory Engine — API Reference (Scalar)");
    // Only same-origin paths; "//host" and "/\host" are protocol-relative in browsers
    let spec_url = params
        .spec_url
        .as_deref()
        .filter(|u| u.starts_with('/') && !u.starts_with("//") && !u.contains('\\'))
        .unwrap_or("/api/v1/openapi.json");
    // Theme and layout sit inside a JSON string in a single-quoted attribute, so only known values are accepted
    let theme = params
        .theme
        .as_deref()
        .filter(|t| SCALAR_THEMES.contains(t))
        .unwrap_or("solarized");
    let layout = params
        .layout
        .as_deref()
        .filter(|l| SCALAR_LAYOUTS.contains(l))
        .unwrap_or("modern");

    let rendered = SCALAR_TEMPLATE
        .replace("{{TITLE}}", &escape_html(title))
        .replace("{{SPEC_URL}}", &escape_html(spec_url))
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
    let frontend_healthy = tokio::fs::try_exists(&primary_dist).await.unwrap_or(false)
        || tokio::fs::try_exists("frontend/dist/index.html").await.unwrap_or(false)
        || tokio::fs::try_exists("/app/frontend/dist/index.html").await.unwrap_or(false);

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

const LICENSES_HTML: &str = include_str!("../../templates/licenses.html");
const LICENSES_MD: &str = include_str!("../../THIRD_PARTY_LICENSES.md");

#[derive(Debug, Deserialize, Default)]
pub struct LicensesQuery {
    pub format: Option<String>,
}

/// Serves the comprehensive third-party open-source attribution notices and licenses document.
/// Renders an accessible, interactive HTML page by default to browsers, or raw Markdown when requested.
pub async fn licenses_handler(
    headers: HeaderMap,
    Query(query): Query<LicensesQuery>,
) -> Response {
    use axum::response::IntoResponse;

    let prefers_markdown = query.format.as_deref() == Some("md")
        || headers
            .get("accept")
            .and_then(|h| h.to_str().ok())
            .map(|a| a.contains("text/markdown") && !a.contains("text/html"))
            .unwrap_or(false);

    if prefers_markdown {
        (
            [(axum::http::header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
            LICENSES_MD,
        )
            .into_response()
    } else {
        (
            [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
            LICENSES_HTML,
        )
            .into_response()
    }
}

/// Serves the raw Markdown attribution document directly
pub async fn licenses_md_handler() -> ([(axum::http::HeaderName, &'static str); 1], &'static str) {
    (
        [(axum::http::header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
        LICENSES_MD,
    )
}




