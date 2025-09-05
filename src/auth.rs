use axum::{
    http::{HeaderMap, StatusCode},
    response::Redirect,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Simple in-memory session store (in production, use Redis or database)
pub type SessionStore = Arc<RwLock<HashMap<String, Session>>>;

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: i64,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

pub fn create_session(user_id: i64, username: String) -> (String, Session) {
    let session_id = Uuid::new_v4().to_string();
    let session = Session {
        user_id,
        username,
        created_at: Utc::now(),
    };
    (session_id, session)
}

pub async fn get_session(
    headers: &HeaderMap,
    sessions: &SessionStore,
) -> Option<Session> {
    if let Some(cookie) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie.to_str() {
            if let Some(session_id) = extract_session_id(cookie_str) {
                let sessions = sessions.read().await;
                return sessions.get(&session_id).cloned();
            }
        }
    }
    None
}

pub fn extract_session_id(cookie: &str) -> Option<String> {
    cookie
        .split(';')
        .find_map(|pair| {
            let pair = pair.trim();
            if pair.starts_with("session_id=") {
                Some(pair[11..].to_string())
            } else {
                None
            }
        })
}

pub async fn require_auth(
    headers: HeaderMap,
    sessions: &SessionStore,
) -> Result<Session, (StatusCode, Redirect)> {
    if let Some(session) = get_session(&headers, sessions).await {
        Ok(session)
    } else {
        Err((StatusCode::UNAUTHORIZED, Redirect::to("/login")))
    }
}
