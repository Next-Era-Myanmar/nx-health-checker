use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub default_username: String,
    pub default_password: String,
    pub host: String,
    pub port: u16,
    pub prometheus_enabled: bool,
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> Self {
        // Load config.env file if it exists
        dotenv::from_filename("config.env").ok();

        let config = Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:health_check.db?mode=rwc".to_string()),
            
            default_username: env::var("DEFAULT_USERNAME")
                .unwrap_or_else(|_| "admin".to_string()),
            
            default_password: env::var("DEFAULT_PASSWORD")
                .unwrap_or_else(|_| "admin".to_string()),
            
            host: env::var("HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            
            port: env::var("PORT")
                .unwrap_or_else(|_| "3030".to_string())
                .parse()
                .unwrap_or(3030),
            
            prometheus_enabled: env::var("PROMETHEUS_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            
            rust_log: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
        };
        
        config
    }

    pub fn socket_addr(&self) -> std::net::SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .unwrap_or_else(|_| "127.0.0.1:3030".parse().unwrap())
    }
}
