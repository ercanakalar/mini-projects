use sqlx::{Error as SqlxError, PgPool, query_as};
use uuid::Uuid;

use crate::domain::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, SqlxError> {
        let user = query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE id = $1
            AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_nick_name(
        pool: &PgPool,
        nick_name: &str,
        exclude_user_id: Uuid,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE nick_name = $1
            AND id != $2
            AND deleted_at IS NULL
            "#,
        )
        .bind(nick_name)
        .bind(exclude_user_id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        nick_name: Option<String>,
        photo: Option<String>,
    ) -> Result<User, SqlxError> {
        let user = query_as::<_, User>(
            r#"
            UPDATE users
            SET
                first_name = COALESCE($1, first_name),
                last_name  = COALESCE($2, last_name),
                nick_name  = COALESCE($3, nick_name),
                photo      = COALESCE($4, photo),
                updated_at = NOW()
            WHERE id = $5
            AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(first_name)
        .bind(last_name)
        .bind(nick_name)
        .bind(photo)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, SqlxError> {
        let users = query_as::<_, User>(
            r#"
        SELECT * FROM users
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn find_permit_by_user_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<String>, sqlx::Error> {
        let row: Option<(String,)> = query_as(
            r#"
        SELECT p.name FROM permits p
        INNER JOIN users u ON u.permit_id = p.id
        WHERE u.id = $1
        AND u.deleted_at IS NULL
        "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|(name,)| name))
    }

    pub async fn update_permit(
        pool: &PgPool,
        user_id: &Uuid,
        permit_id: &Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE users
        SET permit_id = $1,
            updated_at = NOW()
        WHERE id = $2
        "#,
        )
        .bind(permit_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
