use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{
    dto::auth::{
        api_response::ApiResponse, create_user::CreateUserRequest,
        forgot_password_request::ForgotPasswordRequest, login_request::LoginRequest,
        refresh_token_request::RefreshTokenRequest, reset_password_request::ResetPasswordRequest,
    },
    errors::app_error::AppError,
    services::auth_service::AuthService,
    state::AppState,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let data = AuthService::sign_up(&state.db, request.email, request.password).await?;
    Ok(Json(ApiResponse::success("Sign up successfully!", data)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let data = AuthService::sign_in(&state.db, request.email, request.password).await?;
    Ok(Json(ApiResponse::success("Sign in successfully!", data)))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(request): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    AuthService::forgot_password(&state.db, &state.email_service, request.email).await?;
    Ok(Json(ApiResponse::<()>::empty("Password reset email sent")))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    AuthService::reset_password(&state.db, token, body.password).await?;
    Ok(Json(ApiResponse::<()>::empty("Password reset successful")))
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(body): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let data = AuthService::refresh_token(&state.db, body.refresh_token).await?;
    Ok(Json(ApiResponse::success(
        "Token refreshed successfully!",
        data,
    )))
}

pub async fn logout(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, AppError> {
    AuthService::logout(&state.db, auth.token()).await?;
    Ok(Json(ApiResponse::<()>::empty("Logged out successfully")))
}
