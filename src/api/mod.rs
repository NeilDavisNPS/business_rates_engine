pub mod handlers;
pub mod middleware;

use axum::{
    routing::{get, post},
    Router, middleware::from_fn,
};
use crate::models::auth::AppState;
use self::handlers::{
    calculate_liability_handler, component_ui_handler, get_reference_codes_handler,
    static_frontend_asset,
};
use self::middleware::owasp_security_headers;

pub fn create_router(state: AppState) -> Router {
    let api_routes: Router<AppState> = Router::new()
        .route("/business-rates/calculate-liability", post(calculate_liability_handler))
        .route("/business-rates/reference-codes", get(get_reference_codes_handler))
        .route("/reference-codes", get(get_reference_codes_handler));

    Router::new()
        .route("/", get(component_ui_handler))
        .route("/admin", get(component_ui_handler))
        .route("/component-ui", get(component_ui_handler))
        .route("/app", get(component_ui_handler))
        .route("/assets/:filename", get(static_frontend_asset))
        .nest("/api/v1", api_routes)
        .layer(from_fn(owasp_security_headers))
        .with_state(state)
}
