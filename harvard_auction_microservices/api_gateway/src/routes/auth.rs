// api_gateway/src/routes/auth.rs

use axum::{routing::post, Json, Router, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::error;
use crate::AppState;

// 1. gRPC Client aur Proto Messages import kar rahe hain
use shared_protos::auth::auth_service_client::AuthServiceClient;
use shared_protos::auth::{LoginRequest as GrpcLoginReq, RegisterRequest as GrpcRegReq};

// 2. React se aane wala JSON Data Model
#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup_handler))
        .route("/login", post(login_handler))
}

// 3. Signup Handler (Axum -> gRPC)
async fn signup_handler(Json(payload): Json<AuthPayload>) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // A. gRPC Server (Auth Service) ko call milao (Port 50051)
    let mut client = AuthServiceClient::connect("http://[::1]:50051")
        .await
        .map_err(|e| {
            error!("gRPC Auth Server down hai: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Auth Service Offline".to_string())
        })?;

    // B. React ke JSON ko gRPC ke Message mein badlo
    let request = tonic::Request::new(GrpcRegReq {
        username: payload.username,
        password: payload.password,
    });

    // C. gRPC Server se jawab lo
    let response = client.register(request).await.map_err(|e| {
        (StatusCode::BAD_REQUEST, e.message().to_string()) // Agar username pehle se ho
    })?;

    let inner = response.into_inner();

    // D. Wapas React ko JSON bhej do
    Ok(Json(AuthResponse {
        success: inner.success,
        message: inner.message,
        token: None,
    }))
}

// 4. Login Handler (Axum -> gRPC)
async fn login_handler(Json(payload): Json<AuthPayload>) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let mut client = AuthServiceClient::connect("http://[::1]:50051")
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Auth Service Offline".to_string()))?;

    let request = tonic::Request::new(GrpcLoginReq {
        username: payload.username,
        password: payload.password,
    });

    let response = client.login(request).await.map_err(|e| {
        (StatusCode::UNAUTHORIZED, e.message().to_string()) // Ghalat password
    })?;

    let inner = response.into_inner();

    Ok(Json(AuthResponse {
        success: inner.success,
        message: inner.message,
        token: Some(inner.token), // JWT Token wapas bhej diya
    }))
}