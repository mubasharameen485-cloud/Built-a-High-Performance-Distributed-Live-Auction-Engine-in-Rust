// auth_service/src/main.rs

mod db;
mod service;

use dotenvy::dotenv;
use tonic::transport::Server;
use tracing::info;

// Apni banayi hui service import kar rahe hain
use service::MyAuthService;
use shared_protos::auth::auth_service_server::AuthServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    info!("Starting Auth gRPC Server...");

    // 1. Database Connect
    let pool = db::connect_and_setup().await;

    // 2. Apni Service ko State (Database) dena
    let auth_service = MyAuthService { pool };

    // 3. gRPC Server ka Address (Port 50051)
    let addr = "[::1]:50051".parse()?;
    info!("Auth gRPC Server is listening on {}", addr);

    // 4. Server Start
    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}