use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::SqlitePool;
use crate::auth::SessionStore;
use crate::collector::GLOBAL_COLLECTOR;
use crate::handlers;
use tower_http::services::ServeDir;

pub fn create_router(pool: SqlitePool, sessions: SessionStore) -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .route("/login", get(handlers::login_page))
        .route("/login", post(handlers::login))
        .route("/dashboard", get(handlers::dashboard))
        .route("/change-password", get(handlers::change_password_page))
        .route("/logout", get(handlers::logout))
        // Change password route
        .route("/api/change-password", post(handlers::change_password))
        // Service management routes
        .route("/api/services", get(handlers::get_services))
        .route("/api/services", post(handlers::create_service))
        .route("/api/services/:id", put(handlers::update_service))
        .route("/api/services/:id", delete(handlers::delete_service))
        // Health check routes
        .route("/health", get(handlers::health_check))
        .route("/api/services/:id/health", get(handlers::check_service_health))
        .route("/api/services/health", get(handlers::check_all_services_health))
        .route("/metrics", get(handlers::prometheus_metrics))
        .route("/api/metrics/restart", post({
            let pool2 = pool.clone();
            move || async move {
                GLOBAL_COLLECTOR.restart(pool2.clone()).await;
                axum::Json(serde_json::json!({"success": true, "message": "Collectors restarted"}))
            }
        }))
        .nest_service("/assets", ServeDir::new("static/assets"))
        .with_state((pool, sessions))
}
