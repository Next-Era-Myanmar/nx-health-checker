use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DashboardData {
    pub username: String,
    pub login_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Service {
    pub id: i64,
    pub service_name: String,
    pub healthcheck_url: String,
    pub healthcheck_duration_seconds: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateServiceRequest {
    pub service_name: String,
    pub healthcheck_url: String,
    pub healthcheck_duration_seconds: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServiceRequest {
    pub service_name: Option<String>,
    pub healthcheck_url: Option<String>,
    pub healthcheck_duration_seconds: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize)]
pub struct ChangePasswordResponse {
    pub success: bool,
    pub message: String,
}
