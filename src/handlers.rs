

use axum::{
    extract::{State, Form, Path},
    http::{HeaderMap, StatusCode, header},
    response::{Html, Response, Json},
};
use sqlx::{SqlitePool, Row};
use crate::metrics::{SERVICE_STATUS, SERVICE_LATENCY_SECONDS, SERVICE_CHECKS_TOTAL};
use std::fs;
use crate::{auth::{SessionStore, create_session, require_auth, extract_session_id, get_session}, models::{LoginRequest, LoginResponse, Service, CreateServiceRequest, UpdateServiceRequest, ChangePasswordRequest, ChangePasswordResponse}};

pub async fn index(
    State((_pool, sessions)): State<(SqlitePool, SessionStore)>,
    headers: HeaderMap,
) -> Result<Html<String>, Response> {
    // Check if user is already authenticated
    if let Some(_session) = get_session(&headers, &sessions).await {
        // User is already logged in, redirect to dashboard
        return Err(Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", "/dashboard")
            .body(axum::body::Body::from("Redirecting to dashboard..."))
            .unwrap()
            .into());
    }
    
    // User is not authenticated, show index page
    let html = fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| include_str!("../static/index.html").to_string());
    Ok(Html(html))
}

pub async fn login_page(
    State((_pool, sessions)): State<(SqlitePool, SessionStore)>,
    headers: HeaderMap,
) -> Result<Html<String>, Response> {
    // Check if user is already authenticated
    if let Some(_session) = get_session(&headers, &sessions).await {
        // User is already logged in, redirect to dashboard
        return Err(Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", "/dashboard")
            .body(axum::body::Body::from("Redirecting to dashboard..."))
            .unwrap()
            .into());
    }
    
    // User is not authenticated, show login page
    let html = fs::read_to_string("static/login.html")
        .unwrap_or_else(|_| include_str!("../static/login.html").to_string());
    Ok(Html(html))
}

pub async fn login(
    State((pool, sessions)): State<(SqlitePool, SessionStore)>,
    Form(login_data): Form<LoginRequest>,
) -> Response {
    // Query user from database
    match sqlx::query("SELECT * FROM users WHERE username = ? AND password = ?")
        .bind(&login_data.username)
        .bind(&login_data.password)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(row)) => {
            // Create session
            let user_id: i64 = row.get("id");
            let username: String = row.get("username");
            let (session_id, session) = create_session(user_id, username.clone());
            sessions.write().await.insert(session_id.clone(), session);
            
            // Create response with cookie
            let response_data = LoginResponse {
                success: true,
                message: "Login successful!".to_string(),
                redirect_url: Some("/dashboard".to_string()),
            };
            
            // Add session cookie
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Set-Cookie", format!("session_id={}; Path=/; HttpOnly", session_id))
                .body(axum::body::Body::from(serde_json::to_string(&response_data).unwrap()))
                .unwrap();
            
            response
        }
        Ok(None) => {
            Response::builder()
                .status(StatusCode::OK)
                .body(axum::body::Body::from(
                    serde_json::to_string(&LoginResponse {
                        success: false,
                        message: "Invalid username or password".to_string(),
                        redirect_url: None,
                    }).unwrap()
                ))
                .unwrap()
        }
        Err(_) => {
            Response::builder()
                .status(StatusCode::OK)
                .body(axum::body::Body::from(
                    serde_json::to_string(&LoginResponse {
                        success: false,
                        message: "Database error occurred".to_string(),
                        redirect_url: None,
                    }).unwrap()
                ))
                .unwrap()
        }
    }
}

pub async fn dashboard(
    State((_pool, sessions)): State<(SqlitePool, SessionStore)>,
    headers: HeaderMap,
) -> Result<Html<String>, Response> {
    // Check authentication
    let session = match require_auth(headers, &sessions).await {
        Ok(session) => session,
        Err((_status, _redirect)) => {
            // Return a redirect response instead of an error
            return Err(Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/login")
                .body(axum::body::Body::from("Redirecting to login..."))
                .unwrap()
                .into());
        }
    };

    // Read the dashboard template
    let mut html = fs::read_to_string("static/dashboard.html")
        .unwrap_or_else(|_| include_str!("../static/dashboard.html").to_string());
    
    // Replace template variables
    html = html.replace("{{username}}", &session.username);
    html = html.replace("{{login_time}}", &session.created_at.format("%Y-%m-%d %H:%M:%S").to_string());

    Ok(Html(html))
}

pub async fn change_password_page(
    State((_pool, sessions)): State<(SqlitePool, SessionStore)>,
    headers: HeaderMap,
) -> Result<Html<String>, Response> {
    // Check authentication
    let _session = match require_auth(headers, &sessions).await {
        Ok(session) => session,
        Err((_status, _redirect)) => {
            // Return a redirect response instead of an error
            return Err(Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/login")
                .body(axum::body::Body::from("Redirecting to login..."))
                .unwrap()
                .into());
        }
    };

    // Read the change password template
    let html = fs::read_to_string("static/change-password.html")
        .unwrap_or_else(|_| include_str!("../static/change-password.html").to_string());

    Ok(Html(html))
}

pub async fn logout(
    State((_pool, sessions)): State<(SqlitePool, SessionStore)>,
    headers: HeaderMap,
) -> Response {
    // Extract session ID from cookie and remove it
    if let Some(cookie) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie.to_str() {
            if let Some(session_id) = extract_session_id(cookie_str) {
                sessions.write().await.remove(&session_id);
            }
        }
    }
    
    // Redirect to home page with cookie removal
    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/")
        .header("Set-Cookie", "session_id=; Path=/; HttpOnly; Max-Age=0")
        .body(axum::body::Body::from("Redirecting to home page..."))
        .unwrap()
}

// Service management functions
pub async fn get_services(
    State((pool, _sessions)): State<(SqlitePool, SessionStore)>,
) -> Result<Json<Vec<Service>>, Response> {
    match sqlx::query_as::<_, Service>("SELECT * FROM services ORDER BY service_name")
        .fetch_all(&pool)
        .await
    {
        Ok(services) => Ok(Json(services)),
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to fetch services"))
            .unwrap()
            .into()),
    }
}

pub async fn create_service(
    State((pool, _sessions)): State<(SqlitePool, SessionStore)>,
    Json(service_data): Json<CreateServiceRequest>,
) -> Result<Json<serde_json::Value>, Response> {
    let now = chrono::Utc::now();
    
    // First insert the service
    match sqlx::query(
        "INSERT INTO services (service_name, healthcheck_url, healthcheck_duration_seconds, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&service_data.service_name)
    .bind(&service_data.healthcheck_url)
    .bind(service_data.healthcheck_duration_seconds)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    {
        Ok(_) => {
            // Then get the last inserted ID
            match sqlx::query("SELECT last_insert_rowid() as id")
                .fetch_one(&pool)
                .await
            {
                Ok(row) => {
                    let service_id: i64 = row.get("id");
                    Ok(Json(serde_json::json!({
                        "success": true,
                        "message": "Service created successfully",
                        "id": service_id
                    })))
                },
                Err(_) => Err(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(axum::body::Body::from("Failed to get service ID"))
                    .unwrap()
                    .into()),
            }
        },
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to create service"))
            .unwrap()
            .into()),
    }
}

pub async fn update_service(
    State((pool, _sessions)): State<(SqlitePool, SessionStore)>,
    Path(service_id): Path<i64>,
    Json(service_data): Json<UpdateServiceRequest>,
) -> Result<Json<serde_json::Value>, Response> {
    let now = chrono::Utc::now();
    
    // Update each field individually if provided
    if let Some(name) = &service_data.service_name {
        if let Err(_) = sqlx::query("UPDATE services SET service_name = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(now)
            .bind(service_id)
            .execute(&pool)
            .await
        {
            return Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Failed to update service name"))
                .unwrap()
                .into());
        }
    }
    
    if let Some(url) = &service_data.healthcheck_url {
        if let Err(_) = sqlx::query("UPDATE services SET healthcheck_url = ?, updated_at = ? WHERE id = ?")
            .bind(url)
            .bind(now)
            .bind(service_id)
            .execute(&pool)
            .await
        {
            return Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Failed to update service URL"))
                .unwrap()
                .into());
        }
    }
    
    if let Some(duration) = service_data.healthcheck_duration_seconds {
        if let Err(_) = sqlx::query("UPDATE services SET healthcheck_duration_seconds = ?, updated_at = ? WHERE id = ?")
            .bind(duration)
            .bind(now)
            .bind(service_id)
            .execute(&pool)
            .await
        {
            return Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Failed to update service duration"))
                .unwrap()
                .into());
        }
    }
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Service updated successfully"
    })))
}

pub async fn delete_service(
    State((pool, _sessions)): State<(SqlitePool, SessionStore)>,
    Path(service_id): Path<i64>,
) -> Result<Json<serde_json::Value>, Response> {
    match sqlx::query("DELETE FROM services WHERE id = ?")
        .bind(service_id)
        .execute(&pool)
        .await
    {
        Ok(_) => Ok(Json(serde_json::json!({
            "success": true,
            "message": "Service deleted successfully"
        }))),
        Err(_) => Err(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to delete service"))
            .unwrap()
            .into()),
    }
}

// Health check function for a single service
pub async fn check_service_health(
    State((pool, _sessions)): State<(SqlitePool, SessionStore)>,
    Path(service_id): Path<i64>,
) -> Result<Json<serde_json::Value>, Response> {
    // Get service details from database
    let service = match sqlx::query_as::<_, Service>("SELECT * FROM services WHERE id = ?")
        .bind(service_id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(service)) => service,
        Ok(None) => {
            return Err(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(axum::body::Body::from("Service not found"))
                .unwrap()
                .into());
        }
        Err(_) => {
            return Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Database error"))
                .unwrap()
                .into());
        }
    };

    // Perform health check and record metrics
    let start = std::time::Instant::now();
    let status = match perform_health_check(&service.healthcheck_url).await {
        Ok(status) => status,
        Err(_) => "DOWN".to_string(),
    };
    let elapsed = start.elapsed().as_secs_f64();
    let url_label = service.healthcheck_url.clone();
    let name_label = service.service_name.clone();
    let labels = [&url_label[..], &name_label[..]];
    SERVICE_LATENCY_SECONDS.with_label_values(&labels).set(elapsed);
    SERVICE_STATUS.with_label_values(&labels).set(if status == "UP" { 1.0 } else { 0.0 });
    SERVICE_CHECKS_TOTAL.with_label_values(&[labels[0], labels[1], if status == "UP" { "UP" } else { "DOWN" }]).inc();

    Ok(Json(serde_json::json!({
        "service_id": service_id,
        "service_name": service.service_name,
        "status": status,
        "checked_at": chrono::Utc::now()
    })))
}

// Health check function for all services
pub async fn check_all_services_health(
    State((pool, _sessions)): State<(SqlitePool, SessionStore)>,
) -> Result<Json<serde_json::Value>, Response> {
    // Get all services
    let services = match sqlx::query_as::<_, Service>("SELECT * FROM services ORDER BY service_name")
        .fetch_all(&pool)
        .await
    {
        Ok(services) => services,
        Err(_) => {
            return Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Failed to fetch services"))
                .unwrap()
                .into());
        }
    };

    let mut results = Vec::new();
    
    for service in services {
        let start = std::time::Instant::now();
        let status = match perform_health_check(&service.healthcheck_url).await {
            Ok(status) => status,
            Err(_) => "DOWN".to_string(),
        };
        let elapsed = start.elapsed().as_secs_f64();
        let url_label = service.healthcheck_url.clone();
        let name_label = service.service_name.clone();
        let labels = [&url_label[..], &name_label[..]];
        SERVICE_LATENCY_SECONDS.with_label_values(&labels).set(elapsed);
        SERVICE_STATUS.with_label_values(&labels).set(if status == "UP" { 1.0 } else { 0.0 });
        SERVICE_CHECKS_TOTAL.with_label_values(&[labels[0], labels[1], if status == "UP" { "UP" } else { "DOWN" }]).inc();

        results.push(serde_json::json!({
            "service_id": service.id,
            "service_name": service.service_name,
            "status": status,
            "checked_at": chrono::Utc::now()
        }));
    }

    Ok(Json(serde_json::json!({
        "services": results,
        "total_checked": results.len()
    })))
}

// Helper function to perform HTTP health check
async fn perform_health_check(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await?;
    
    if response.status().is_success() {
        Ok("UP".to_string())
    } else {
        Ok("DOWN".to_string())
    }
}

// Simple health check handler for Docker
pub async fn health_check(
    State((pool, _sessions)): State<(SqlitePool, SessionStore)>,
) -> Result<Json<serde_json::Value>, Response> {
    // Simple database connectivity check
    match sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now(),
            "service": "nx-health-checker"
        }))),
        Err(_) => Err(Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .body(axum::body::Body::from("Database connection failed"))
            .unwrap()
            .into()),
    }
}

// Prometheus metrics handler with proper Content-Type
pub async fn prometheus_metrics() -> Response {
    let metrics_data = crate::metrics::gather_metrics();
    
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")
        .body(axum::body::Body::from(metrics_data))
        .unwrap()
}

// Change password handler
pub async fn change_password(
    State((pool, sessions)): State<(SqlitePool, SessionStore)>,
    headers: HeaderMap,
    Json(password_data): Json<ChangePasswordRequest>,
) -> Result<Json<ChangePasswordResponse>, Response> {
    // Check authentication
    let session = match require_auth(headers, &sessions).await {
        Ok(session) => session,
        Err((_status, _redirect)) => {
            return Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::from("Unauthorized"))
                .unwrap()
                .into());
        }
    };

    // Validate new password confirmation
    if password_data.new_password != password_data.confirm_password {
        return Ok(Json(ChangePasswordResponse {
            success: false,
            message: "New password and confirmation password do not match".to_string(),
        }));
    }

    // Validate new password length
    if password_data.new_password.len() < 6 {
        return Ok(Json(ChangePasswordResponse {
            success: false,
            message: "New password must be at least 6 characters long".to_string(),
        }));
    }

    // Verify current password
    match sqlx::query("SELECT password FROM users WHERE id = ? AND password = ?")
        .bind(session.user_id)
        .bind(&password_data.current_password)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(_)) => {
            // Current password is correct, update to new password
            match sqlx::query("UPDATE users SET password = ? WHERE id = ?")
                .bind(&password_data.new_password)
                .bind(session.user_id)
                .execute(&pool)
                .await
            {
                Ok(_) => {
                    Ok(Json(ChangePasswordResponse {
                        success: true,
                        message: "Password changed successfully!".to_string(),
                    }))
                }
                Err(_) => {
                    Ok(Json(ChangePasswordResponse {
                        success: false,
                        message: "Failed to update password. Please try again.".to_string(),
                    }))
                }
            }
        }
        Ok(None) => {
            Ok(Json(ChangePasswordResponse {
                success: false,
                message: "Current password is incorrect".to_string(),
            }))
        }
        Err(_) => {
            Ok(Json(ChangePasswordResponse {
                success: false,
                message: "Database error occurred. Please try again.".to_string(),
            }))
        }
    }
}