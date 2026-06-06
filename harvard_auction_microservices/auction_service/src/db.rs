// auction_service/src/db.rs

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

    // 1. Items Table banana
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS auction_items (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            current_price DOUBLE PRECISION NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    // 2. Bids Table banana (Kon kis item par kitni boli laga raha hai)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS bids (
            id SERIAL PRIMARY KEY,
            item_id INT NOT NULL REFERENCES auction_items(id),
            user_id INT NOT NULL,
            bid_amount DOUBLE PRECISION NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    info!("Auction Database connected aur Tables ready hain!");
    pool
}