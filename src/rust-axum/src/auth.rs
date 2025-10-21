use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiration time
    pub is_admin: bool,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expire_minutes: i64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_string()),
            jwt_expire_minutes: env::var("JWT_EXPIRE_MINUTES")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
        }
    }
}

pub async fn hash_password(password: &str) -> Result<String, AppError> {
    let password = password.to_string();
    // Using bcrypt with cost 8 for consistency with Python implementation
    // Offload CPU-intensive bcrypt to a blocking thread to avoid blocking the async runtime
    tokio::task::spawn_blocking(move || bcrypt::hash(&password, 8))
        .await
        .map_err(|_| AppError::InternalServerError("Task join error".to_string()))?
        .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let password = password.to_string();
    let hash = hash.to_string();
    // Offload CPU-intensive bcrypt to a blocking thread to avoid blocking the async runtime
    tokio::task::spawn_blocking(move || bcrypt::verify(&password, &hash))
        .await
        .map_err(|_| AppError::InternalServerError("Task join error".to_string()))?
        .map_err(|_| AppError::InternalServerError("Failed to verify password".to_string()))
}

pub fn create_token(user_id: &Uuid, is_admin: bool, config: &AuthConfig) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(config.jwt_expire_minutes))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        is_admin,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|_| AppError::InternalServerError("Failed to create token".to_string()))
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;  // Still validate expiration
    validation.validate_nbf = false; // Skip not-before validation for speed
    validation.validate_aud = false; // Skip audience validation for speed
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| {
        tracing::debug!("Token decode error: {:?}", e);
        AppError::Unauthorized("Invalid token".to_string())
    })
}

pub fn extract_token_from_headers(headers: &HeaderMap) -> Result<String, AppError> {
    let auth_header = headers
        .get("authorization")
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid authorization format".to_string()));
    }

    Ok(auth_header[7..].to_string())
}

// Middleware for extracting user ID from JWT token
pub async fn auth_middleware(
    State(auth_config): State<AuthConfig>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token_from_headers(request.headers())?;
    let claims = decode_token(&token, &auth_config.jwt_secret)?;
    
    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}
