use chrono::Utc;
use sqlx::PgPool;

use crate::{
    dto::auth::{
        email_payload::EmailPayload, login_response::LoginResponse,
        refresh_token_response::RefreshTokenResponse, signup_response::SignupResponse,
    },
    errors::app_error::AppError,
    repositories::{
        auth_repository::AuthRepository, password_reset_repository::PasswordResetRepository,
        permit_repository::PermitRepository,
    },
    services::{
        email_service::EmailService, jwt_service::JwtService, password_service::PasswordService,
    },
};

pub struct AuthService;

impl AuthService {
    pub async fn sign_up(
        pool: &PgPool,
        email: String,
        password: String,
    ) -> Result<SignupResponse, AppError> {
        let existing = AuthRepository::find_by_email(pool, &email)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if existing.is_some() {
            return Err(AppError::Conflict("User already exists".into()));
        }

        let permit_id = AuthRepository::find_permit_by_name(pool, "USER")
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| {
                AppError::Internal(
                    "Default USER permit not found. Please seed the database.".into(),
                )
            })?;

        let permissions = PermitRepository::find_permissions_by_permit_id(pool, permit_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let password_hash =
            PasswordService::hash(&password).map_err(|e| AppError::Internal(e.to_string()))?;

        let (user, manuel_auth, tokens) =
            AuthRepository::create(pool, email, password_hash, permit_id, permissions)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(SignupResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            email: manuel_auth.email,
            user_id: user.id,
        })
    }

    pub async fn sign_in(
        pool: &PgPool,
        email: String,
        password: String,
    ) -> Result<LoginResponse, AppError> {
        let user = AuthRepository::find_by_email(pool, &email)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

        if user.token_id.is_none() {
            return Err(AppError::Internal("User token record is missing".into()));
        }

        if !PasswordService::verify(&password, &user.password) {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }
        let permissions = AuthRepository::find_permissions_by_user_id(pool, user.id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let token_pair = JwtService::generate_tokens(&user.id.to_string(), permissions)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        AuthRepository::save_tokens(
            pool,
            user.id,
            &token_pair.access_token,
            &token_pair.refresh_token,
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(LoginResponse {
            access_token: Some(token_pair.access_token),
            refresh_token: Some(token_pair.refresh_token),
            email: user.email,
            id: user.id,
        })
    }

    pub async fn refresh_token(
        pool: &PgPool,
        refresh_token: String,
    ) -> Result<RefreshTokenResponse, AppError> {
        let claims = JwtService::verify(&refresh_token)
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".into()))?;

        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Internal("Invalid user id in token".into()))?;

        let user = AuthRepository::find_by_id(pool, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        if user.token_id.is_none() {
            return Err(AppError::NotFound("Token record not found".into()));
        }

        let permissions = AuthRepository::find_permissions_by_user_id(pool, user.id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let token_pair = JwtService::generate_tokens(&user.id.to_string(), permissions)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        AuthRepository::save_tokens(
            pool,
            user.id,
            &token_pair.access_token,
            &token_pair.refresh_token,
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(RefreshTokenResponse {
            user_id: user.id,
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
        })
    }

    pub async fn forgot_password(
        pool: &PgPool,
        email_service: &EmailService,
        email: String,
    ) -> Result<(), AppError> {
        let user = AuthRepository::find_by_email(pool, &email)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        if user.token_id.is_none() {
            return Err(AppError::NotFound("User token record not found".into()));
        }

        let reset_token = uuid::Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::minutes(15);
        let frontend_url =
            std::env::var("FRONTEND_URL").map_err(|e| AppError::Internal(e.to_string()))?;
        let reset_link = format!("{}/reset-password/{}", frontend_url, reset_token);

        PasswordResetRepository::create_token(pool, user.id, &reset_token, expires_at)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        email_service
            .send_email(EmailPayload {
                to: user.email.clone(),
                subject: "Reset Password".into(),
                html: format!(
                    r#"
                    <h2>Password Reset Request</h2>
                    <p>Click the link below to reset your password. This link expires in 15 minutes.</p>
                    <a href="{0}">Reset Password</a>
                    <p>If you didn't request this, please ignore this email.</p>
                    "#,
                    reset_link
                ),
                text: Some(format!(
                    "Reset your password: {}\nThis link expires in 15 minutes.\nIf you didn't request this, please ignore this email.",
                    reset_link
                )),
                cc: None,
            })
            .await?;

        Ok(())
    }

    pub async fn reset_password(
        pool: &PgPool,
        token: String,
        new_password: String,
    ) -> Result<(), AppError> {
        let record = PasswordResetRepository::find_by_token(pool, &token)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("Invalid or expired token".into()))?;

        let password_hash =
            PasswordService::hash(&new_password).map_err(|e| AppError::Internal(e.to_string()))?;

        let user = AuthRepository::find_by_id(pool, record.user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        AuthRepository::update_password(pool, user.auth_id, &password_hash)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        PasswordResetRepository::clear_reset_token(pool, record.user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn logout(pool: &PgPool, token: &str) -> Result<(), AppError> {
        let claims = JwtService::verify(token)
            .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Internal("Invalid user id in token".into()))?;

        AuthRepository::clear_tokens(pool, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
