use sqlx::PgPool;

use crate::services::email_service::EmailService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub email_service: EmailService,
}
