use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod database;
mod metrics;
mod collector;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // Load configuration from environment
    let config = config::Config::from_env();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let pool = database::init_database(&config.database_url, &config.default_username, &config.default_password)
        .await
        .expect("Failed to initialize database");

    // Initialize session store
    let sessions = auth::SessionStore::default();

    // Start collectors
    collector::GLOBAL_COLLECTOR.start(pool.clone()).await;

    // Create the application router
    let app = routes::create_router(pool.clone(), sessions)
        .layer(CorsLayer::permissive());

    // Run the application
    let addr = config.socket_addr();
    tracing::info!("Listening on http://{}", addr);
    
    if config.prometheus_enabled {
        tracing::info!("Prometheus metrics enabled at: {}/metrics", addr);
    }

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
