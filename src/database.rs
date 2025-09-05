use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, Row};
use tracing::info;

pub async fn init_database(database_url: &str, username: &str, password: &str) -> Result<SqlitePool, sqlx::Error> {
    // The 'c' flag in mode=rwc tells SQLite to create the database if it doesn't exist
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Create tables directly
    create_tables(&pool).await?;
    
    // Initialize admin user if not exists
    init_admin_user(&pool, username, password).await?;
    
    // Initialize sample services if none exist
    init_sample_services(&pool).await?;
    
    info!("Database initialized successfully");
    Ok(pool)
}

async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            created_at DATETIME NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    // Create index on username for faster lookups
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)")
        .execute(pool)
        .await?;

    // Create services table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            service_name TEXT NOT NULL,
            healthcheck_url TEXT NOT NULL,
            healthcheck_duration_seconds INTEGER NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    // Create index on service_name for faster lookups
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_services_name ON services(service_name)")
        .execute(pool)
        .await?;

    Ok(())
}

async fn init_sample_services(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Check if any services exist
    let services_count = sqlx::query("SELECT COUNT(*) as count FROM services")
        .fetch_one(pool)
        .await?
        .get::<i64, _>("count");

    if services_count == 0 {
        // Create sample services
        let now = chrono::Utc::now();
        
        // Sample service 1: Google
        sqlx::query(
            "INSERT INTO services (service_name, healthcheck_url, healthcheck_duration_seconds, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind("Google")
        .bind("https://www.google.com")
        .bind(30)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        
        info!("Sample services created: Google");
    }

    Ok(())
}

async fn init_admin_user(pool: &SqlitePool, username: &str, password: &str) -> Result<(), sqlx::Error> {
    // Check if admin user exists
    let admin_exists = sqlx::query("SELECT COUNT(*) as count FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?
        .get::<i64, _>("count");

    if admin_exists == 0 {
        // Create admin user
        sqlx::query(
            "INSERT INTO users (username, password, created_at) VALUES (?, ?, ?)"
        )
        .bind(username)
        .bind(password)
        .bind(chrono::Utc::now())
        .execute(pool)
        .await?;
        
        info!("Admin user created with username: {}, password: {}", username, password);
    }

    Ok(())
}
