use axum::Router;
use dotenvy::dotenv;
use routes::auth_routes::auth_routes;
use sqlx::PgPool;

mod domain;
mod dto;
mod errors;
mod extractors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use crate::{
    errors::app_error::AppError, routes::user_routes::user_routes,
    services::email_service::EmailService,
};
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").map_err(|e| AppError::Internal(e.to_string()))?;

    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|e: sqlx::Error| AppError::Database(e.to_string()))?;

    let email_service = EmailService::new()?;
    email_service.verify_connection().await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let state = AppState {
        db: pool,
        email_service,
    };

    let app = Router::new()
        .nest("/api/auth", auth_routes(state.clone()))
        .nest("/api/user", user_routes(state.clone()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(())
}
