use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::{errors::app_error::AppError, extractors::jwt_claim::Claims};

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct JwtService;

impl JwtService {
    fn jwt_secret() -> Result<String, AppError> {
        std::env::var("JWT_SECRET").map_err(|_| AppError::Internal("JWT_SECRET not set".into()))
    }

    fn access_token_expiry() -> i64 {
        std::env::var("ACCESS_TOKEN_EXPIRATION")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(3600)
    }

    fn refresh_token_expiry() -> i64 {
        std::env::var("REFRESH_TOKEN_EXPIRATION")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(604800)
    }

    pub fn generate_tokens(user_id: &str, permissions: Vec<String>) -> Result<TokenPair, AppError> {
        let access_token = Self::generate_access_token(user_id, permissions)?;

        let refresh_token = Self::generate_refresh_token(user_id)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    pub fn generate_access_token(
        user_id: &str,
        permissions: Vec<String>,
    ) -> Result<String, AppError> {
        Self::generate_token(user_id, Self::access_token_expiry(), permissions)
    }

    pub fn generate_refresh_token(user_id: &str) -> Result<String, AppError> {
        Self::generate_token(user_id, Self::refresh_token_expiry(), vec![])
    }

    fn generate_token(
        user_id: &str,
        expiry_seconds: i64,
        permissions: Vec<String>,
    ) -> Result<String, AppError> {
        let secret = Self::jwt_secret()?;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: (Utc::now() + Duration::seconds(expiry_seconds)).timestamp() as usize,
            permissions,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::Unauthorized(e.to_string()))
    }

    pub fn verify(token: &str) -> Result<Claims, AppError> {
        let secret = Self::jwt_secret()?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        Ok(token_data.claims)
    }
}
