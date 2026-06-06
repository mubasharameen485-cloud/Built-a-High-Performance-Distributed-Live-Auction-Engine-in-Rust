// auth_service/src/db.rs

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tracing::info;

pub async fn connect_and_setup() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL .env mein nahi hai");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    // Table banana (Agar pehle se nahi hai)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            role VARCHAR(50) NOT NULL DEFAULT 'user'
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    info!("Auth Database connected aur Table ready hai!");
    pool
}