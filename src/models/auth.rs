use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUser {
    pub user_id: String,
    pub email: String,
    pub role: String,
}

#[derive(Clone)]
pub struct AppState {
    pub repo: crate::db::repository::ParameterRepository,
}
