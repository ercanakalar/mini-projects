use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    dto::{
        auth::api_response::ApiResponse,
        user::{
            update_permit_request::UpdatePermitRequest, update_user_request::UpdateUserRequest,
        },
    },
    errors::app_error::AppError,
    extractors::auth_claims::AuthClaims,
    services::user_service::UserService,
    state::AppState,
};

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let data = UserService::get_user_by_id(&state.db, user_id).await?;
    Ok(Json(ApiResponse::success(
        "User fetched successfully",
        data,
    )))
}

pub async fn update_user(
    AuthClaims(claims): AuthClaims,
    State(state): State<AppState>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Internal("Invalid user id in token".into()))?;

    let data = UserService::update_user(&state.db, user_id, body).await?;
    Ok(Json(ApiResponse::success(
        "User updated successfully",
        data,
    )))
}

pub async fn get_all_users(
    AuthClaims(claims): AuthClaims,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let requester_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Internal("Invalid user id in token".into()))?;

    let data = UserService::get_all_users(&state.db, requester_id).await?;
    Ok(Json(ApiResponse::success(
        "Users fetched successfully",
        data,
    )))
}

pub async fn update_permit_by_user_id(
    State(state): State<AppState>,
    Json(body): Json<UpdatePermitRequest>,
) -> Result<impl IntoResponse, AppError> {
    let data =
        UserService::update_permit_by_user_id(&state.db, body.user_id, body.permit_id).await?;
    Ok(Json(ApiResponse::success(
        "Updated permission of the user",
        data,
    )))
}
