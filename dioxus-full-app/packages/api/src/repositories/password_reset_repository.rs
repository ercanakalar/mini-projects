use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::auth::Tokens;

pub struct PasswordResetRepository;

impl PasswordResetRepository {
    pub async fn create_token(
        pool: &PgPool,
        user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        let result = sqlx::query(
            r#"
        UPDATE tokens
        SET reset_token = $1,
            password_reset_token_expiry = $2,
            updated_at = NOW()
        WHERE user_id = $3
        "#,
        )
        .bind(token)
        .bind(expires_at)
        .bind(user_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Option<Tokens>, sqlx::Error> {
        let record = sqlx::query_as::<_, Tokens>(
            r#"
            SELECT *
            FROM tokens
            WHERE reset_token = $1
            AND password_reset_token_expiry > NOW()
            AND deleted_at IS NULL
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    pub async fn clear_reset_token(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE tokens
            SET reset_token = NULL,
                password_reset_token_expiry = NULL,
                updated_at = NOW()
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
