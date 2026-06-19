use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{errors::app_error::AppError, extractors::jwt_claim::Claims};

pub async fn require_permission(
    permission: &'static str,
    req: Request<Body>,
    next: Next,
) -> Response {
    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            if claims.permissions.iter().any(|p| p == permission) {
                next.run(req).await
            } else {
                AppError::Forbidden(format!("Missing required permission: {}", permission))
                    .into_response()
            }
        }
        None => AppError::Unauthorized("You have not permission!".into()).into_response(),
    }
}
