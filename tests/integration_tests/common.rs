use std::fs;
use std::path::PathBuf;
use axum::body::Body;
pub use http_body_util::BodyExt;
use business_rates_engine::api::create_router;
use business_rates_engine::db::repository::ParameterRepository;
use business_rates_engine::models::calculation::*;
use business_rates_engine::models::AppState;

pub fn create_test_app() -> axum::Router {
    let repo = ParameterRepository::new_mock();
    let state = AppState { repo };
    create_router(state)
}

pub async fn body_to_string(body: Body) -> String {
    let bytes = body.collect().await.expect("Failed to read response body").to_bytes();
    String::from_utf8(bytes.to_vec()).expect("Response body is not valid UTF-8")
}

pub fn load_fixture(filename: &str) -> LiabilityCalculationRequest {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(filename);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read fixture at {:?}", path));
    serde_json::from_str(&content).expect("Failed to deserialize fixture request JSON")
}
