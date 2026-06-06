// auth_service/src/service.rs

use tonic::{Request, Response, Status};
use sqlx::PgPool;
use tracing::{info, error};
use std::env;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};

// 1. Shared Protos se apne Contracts import karna
use shared_protos::auth::auth_service_server::AuthService;
use shared_protos::auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};

// 2. JWT ke liye Claims Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub role: String,
    pub exp: usize,
}

// 3. gRPC Server ka State (Ismein Database Pool hoga)
pub struct MyAuthService {
    pub pool: PgPool,
}

// 4. Asal Logic (Trait Implementation)
#[tonic::async_trait]
impl AuthService for MyAuthService {
    
    // ==========================================
    // SIGNUP LOGIC
    // ==========================================
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        info!("Naya Signup aaya: {}", req.username);

        // A. Password Hash Banana (Argon2)
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|_| Status::internal("Hashing fail ho gayi"))?
            .to_string();

        // B. Database mein Save karna
        let result = sqlx::query!(
            "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
            req.username,
            password_hash
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(record) => {
                info!("User successfully save ho gaya. ID: {}", record.id);
                Ok(Response::new(RegisterResponse {
                    success: true,
                    message: "User successfully registered!".to_string(),
                    user_id: record.id,
                }))
            }
            Err(e) => {
                if e.to_string().contains("duplicate key") {
                    error!("Duplicate username: {}", req.username);
                    Err(Status::already_exists("Yeh username pehle se majood hai"))
                } else {
                    error!("Database error: {}", e);
                    Err(Status::internal("Server Error"))
                }
            }
        }
    }

    // ==========================================
    // LOGIN LOGIC
    // ==========================================
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        info!("Login attempt: {}", req.username);

        // A. Database se User dhoondna
        let user_record = sqlx::query!(
            "SELECT id, username, password_hash, role FROM users WHERE username = $1",
            req.username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| Status::internal("Database Error"))?;

        let user = match user_record {
            Some(record) => record,
            None => {
                return Err(Status::unauthenticated("Username ya Password ghalat hai"));
            }
        };

        // B. Password Verify karna
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| Status::internal("Hash Error"))?;
            
        let is_valid = Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok();

        if is_valid {
            // C. JWT Token Banana
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET .env mein nahi hai");
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .unwrap()
                .timestamp() as usize;

            let claims = Claims {
                sub: user.id,
                username: user.username,
                role: user.role,
                exp: expiration,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            ).map_err(|_| Status::internal("Token banane mein masla"))?;

            info!("Login Successful. User ID: {}", user.id);
            Ok(Response::new(LoginResponse {
                success: true,
                token, // Lamba JWT Token bhej diya
                message: "Login Successful!".to_string(),
            }))
        } else {
            Err(Status::unauthenticated("Username ya Password ghalat hai"))
        }
    }
}