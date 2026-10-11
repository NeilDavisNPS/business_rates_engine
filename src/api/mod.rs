pub mod handlers;
pub mod middleware;

use axum::{
    routing::{get, post},
    Router, middleware::from_fn,
};
use crate::models::auth::AppState;
use self::handlers::{
    calculate_liability_handler, component_ui_handler, get_all_parameters_handler,
    get_multipliers_handler, get_reference_codes_handler, get_sbrr_thresholds_handler,
    get_sector_reliefs_handler, get_transitional_caps_handler, openapi_json_handler,
    openapi_yaml_handler, scalar_ui_handler, static_frontend_asset,
};
use self::middleware::owasp_security_headers;

pub fn create_router(state: AppState) -> Router {
    let api_routes: Router<AppState> = Router::new()
        .route("/calculate-liability", post(calculate_liability_handler))
        .route("/reference-codes", get(get_reference_codes_handler))
        .route("/parameters", get(get_all_parameters_handler))
        .route("/parameters/multipliers", get(get_multipliers_handler))
        .route("/parameters/sbrr-thresholds", get(get_sbrr_thresholds_handler))
        .route("/parameters/sector-reliefs", get(get_sector_reliefs_handler))
        .route("/parameters/transitional-caps", get(get_transitional_caps_handler))
        .route("/openapi.json", get(openapi_json_handler))
        .route("/openapi.yaml", get(openapi_yaml_handler));

    Router::new()
        .route("/", get(component_ui_handler))
        .route("/admin", get(component_ui_handler))
        .route("/component-ui", get(component_ui_handler))
        .route("/app", get(component_ui_handler))
        .route("/docs", get(scalar_ui_handler))
        .route("/scalar", get(scalar_ui_handler))
        .route("/api-reference", get(scalar_ui_handler))
        .route("/swagger", get(scalar_ui_handler))
        .route("/swagger-ui", get(scalar_ui_handler))
        .route("/openapi.json", get(openapi_json_handler))
        .route("/openapi.yaml", get(openapi_yaml_handler))
        .route("/assets/:filename", get(static_frontend_asset))
        .nest("/api/v1", api_routes)
        .layer(from_fn(owasp_security_headers))
        .with_state(state)
}

