// api_gateway/src/main.rs

mod routes;
mod websocket;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tokio::sync::broadcast;

// AppState: Ismein koi Database nahi hai! Sirf WebSocket ka channel hai.
#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<String>, // Live Bidding updates sab ko bhejne ke liye
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // 1. Broadcast Channel (Radio Station) for Live Bidding
    let (tx, _rx) = broadcast::channel::<String>(100);
    let state = AppState { tx };

    // 2. CORS (Frontend ke liye ijazat)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 3. Router Setup (REST Routes + WebSocket Route)
    let app = Router::new()
        .merge(routes::create_router()) // REST API routes
        .route("/ws", axum::routing::get(websocket::ws_handler)) // Live Bidding socket
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // 4. Server Start
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("API Gateway (Axum Waiter) Port 3000 par chal raha hai...");
    axum::serve(listener, app).await.unwrap();
}