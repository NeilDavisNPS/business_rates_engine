use std::net::SocketAddr;
use tracing::info;
use business_rates_engine::db::repository::ParameterRepository;
use business_rates_engine::models::AppState;
use business_rates_engine::api::create_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let repo = ParameterRepository::new_mock();
    let state = AppState { repo };
    let app = create_router(state);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting Business Rates Calculation Engine on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
