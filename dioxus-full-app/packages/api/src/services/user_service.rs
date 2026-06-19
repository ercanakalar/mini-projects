use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::user::User, dto::user::update_user_request::UpdateUserRequest,
    errors::app_error::AppError, repositories::user_repository::UserRepository,
};

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<crate::domain::user::User, AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        Ok(user)
    }

    pub async fn update_user(
        pool: &PgPool,
        user_id: Uuid,
        body: UpdateUserRequest,
    ) -> Result<User, AppError> {
        if let Some(ref nick_name) = body.nick_name {
            let existing = UserRepository::find_by_nick_name(pool, nick_name, user_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            if existing.is_some() {
                return Err(AppError::Conflict("This Nick Name in use!".into()));
            }
        }

        let user = UserRepository::update(
            pool,
            user_id,
            body.first_name,
            body.last_name,
            body.nick_name,
            body.photo,
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user)
    }

    pub async fn get_all_users(
        pool: &PgPool,
        requester_id: Uuid,
    ) -> Result<Vec<crate::domain::user::User>, AppError> {
        let permit = UserRepository::find_permit_by_user_id(pool, requester_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Forbidden("No permit found".into()))?;

        if permit != "ADMIN" {
            return Err(AppError::Forbidden(
                "Only admins can access this resource".into(),
            ));
        }

        let users = UserRepository::find_all(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(users)
    }

    pub async fn update_permit_by_user_id(
        pool: &PgPool,
        user_id: Uuid,
        permit_id: Uuid,
    ) -> Result<(), AppError> {
        UserRepository::update_permit(pool, &user_id, &permit_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
