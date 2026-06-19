use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::errors::app_error::AppError;
use crate::extractors::jwt_claim::Claims;

pub struct AuthClaims(pub Claims);

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(AuthClaims)
            .ok_or_else(|| AppError::Unauthorized("Missing claims".into()))
    }
}
