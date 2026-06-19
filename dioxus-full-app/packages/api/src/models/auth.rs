use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct ManuelAuth {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub user_id: Uuid,
    pub token_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct GoogleAuth {
    pub id: Uuid,
    pub email: String,
    pub user_id: Uuid,
    pub token_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Tokens {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub reset_token: Option<String>,
    pub password_reset_token_expiry: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
