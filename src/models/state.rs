use crate::db::repository::ParameterRepository;

/// Shared application state passed to Axum HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub repo: ParameterRepository,
}

