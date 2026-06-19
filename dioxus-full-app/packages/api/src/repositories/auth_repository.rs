use sqlx::{Executor, PgPool};
use uuid::Uuid;

use crate::{
    domain::{user::User, user_with_auth::UserWithAuth},
    models::auth::{ManuelAuth, Tokens},
    services::jwt_service::JwtService,
};

pub struct AuthRepository;

impl AuthRepository {
    pub async fn create(
        pool: &PgPool,
        email: String,
        password_hash: String,
        permit_id: Uuid,
        permissions: Vec<String>,
    ) -> Result<(User, ManuelAuth, Tokens), sqlx::Error> {
        let mut tx = pool.begin().await?;

        let user = sqlx::query_as::<_, User>(
            r#"
        INSERT INTO users (id, email, permit_id, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        RETURNING *
        "#,
        )
        .bind(Uuid::new_v4())
        .bind(email.clone())
        .bind(permit_id)
        .fetch_one(&mut *tx)
        .await?;

        let token_pair = JwtService::generate_tokens(&user.id.to_string(), permissions)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        let tokens = Self::save_tokens(
            &mut *tx,
            user.id,
            &token_pair.access_token,
            &token_pair.refresh_token,
        )
        .await?;

        let manuel_auth = sqlx::query_as::<_, ManuelAuth>(
            r#"
        INSERT INTO manuel_auth (id, email, password, user_id, token_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING *
        "#,
        )
        .bind(Uuid::new_v4())
        .bind(email)
        .bind(password_hash)
        .bind(user.id)
        .bind(tokens.id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok((user, manuel_auth, tokens))
    }

    pub async fn find_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<UserWithAuth>, sqlx::Error> {
        let auth = sqlx::query_as::<_, UserWithAuth>(
            r#"
            SELECT
                u.*,
                m.id        AS auth_id,
                m.email,
                m.password,
                m.token_id,
                m.user_id   AS auth_user_id
            FROM manuel_auth m
            INNER JOIN users u ON m.user_id::uuid = u.id::uuid
            WHERE m.email = $1
            AND m.deleted_at IS NULL
            AND u.deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(auth)
    }

    pub async fn find_by_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<UserWithAuth>, sqlx::Error> {
        let auth = sqlx::query_as::<_, UserWithAuth>(
            r#"
            SELECT
                u.*,
                m.id        AS auth_id,
                m.email,
                m.password,
                m.token_id,
                m.user_id   AS auth_user_id
            FROM users u
            INNER JOIN manuel_auth m ON m.user_id::uuid = u.id::uuid
            WHERE u.id = $1
            AND m.deleted_at IS NULL
            AND u.deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(auth)
    }

    pub async fn find_permit_by_name(
        pool: &PgPool,
        name: &str,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let row: Option<(Uuid,)> = sqlx::query_as(
            r#"
            SELECT id FROM permits WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|(id,)| id))
    }

    pub async fn find_permissions_by_user_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<String>, sqlx::Error> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
        SELECT p.name
        FROM permissions p
        INNER JOIN permit_permissions pp ON pp.permission_id = p.id
        INNER JOIN permits per ON per.id = pp.permit_id
        INNER JOIN users u ON u.permit_id = per.id
        WHERE u.id = $1
        AND u.deleted_at IS NULL
        "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(name,)| name).collect())
    }

    pub async fn update_password(
        pool: &PgPool,
        auth_id: Uuid,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE manuel_auth
            SET password = $1, updated_at = NOW()
            WHERE id = $2
            AND deleted_at IS NULL
            "#,
        )
        .bind(password_hash)
        .bind(auth_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn save_tokens<'a, E>(
        executor: E,
        user_id: Uuid,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Tokens, sqlx::Error>
    where
        E: Executor<'a, Database = sqlx::Postgres>,
    {
        let tokens = sqlx::query_as::<_, Tokens>(
            r#"
            INSERT INTO tokens (id, user_id, access_token, refresh_token, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            ON CONFLICT (user_id) DO UPDATE
            SET access_token = EXCLUDED.access_token,
                refresh_token = EXCLUDED.refresh_token,
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(access_token)
        .bind(refresh_token)
        .fetch_one(executor)
        .await?;

        Ok(tokens)
    }

    pub async fn clear_tokens(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE tokens
            SET access_token = NULL,
                refresh_token = NULL,
                updated_at = NOW()
            WHERE user_id = $1::uuid
            "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
