// auction_service/src/main.rs

mod db;
mod service;

use dotenvy::dotenv;
use tonic::transport::Server;
use tracing::info;

// Apni banayi hui service import kar rahe hain
use service::MyAuctionService;
use shared_protos::auction::auction_service_server::AuctionServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    info!("Starting Auction gRPC Server...");

    // 1. Database Connect
    let pool = db::connect_and_setup().await;

    // 2. Apni Service ko State (Database) dena
    let auction_service = MyAuctionService { pool };

    // 3. gRPC Server ka Address (Port 50052 - Ghaur karo yeh Auth se alag hai!)
    let addr = "[::1]:50052".parse()?;
    info!("Auction gRPC Server is listening on {}", addr);

    // 4. Server Start
    Server::builder()
        .add_service(AuctionServiceServer::new(auction_service))
        .serve(addr)
        .await?;

    Ok(())
}