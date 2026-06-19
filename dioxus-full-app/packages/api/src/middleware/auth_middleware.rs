use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;

use crate::{errors::app_error::AppError, services::jwt_service::JwtService, state::AppState};

pub async fn auth_middleware(
    State(_state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t.to_string(),
        None => {
            return AppError::Unauthorized("Missing authorization token".into()).into_response();
        }
    };

    let claims = match JwtService::verify(&token) {
        Ok(c) => c,
        Err(_e) => {
            return AppError::Unauthorized("Invalid or expired token".into()).into_response();
        }
    };

    let now = Utc::now().timestamp() as usize;
    if claims.exp < now {
        return AppError::Unauthorized("Token has expired".into()).into_response();
    }

    req.extensions_mut().insert(claims);

    next.run(req).await
}
