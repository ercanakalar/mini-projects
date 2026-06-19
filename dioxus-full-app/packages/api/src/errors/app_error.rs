use core::fmt;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Conflict(String),
    Unauthorized(String),
    Database(String),
    Internal(String),
    NotFound(String),
    Forbidden(String),
}

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }

    fn message(&self) -> &str {
        match self {
            AppError::Database(msg) => msg,
            AppError::Internal(msg) => msg,
            AppError::Unauthorized(msg) => msg,
            AppError::Conflict(msg) => msg,
            AppError::NotFound(msg) => msg,
            AppError::Forbidden(msg) => msg,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = Json(ApiResponse::<()> {
            success: false,
            message: self.message().to_string(),
            data: None,
        });

        (status, body).into_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
