use sqlx::PgPool;
use uuid::Uuid;

pub struct PermitRepository;

impl PermitRepository {
    pub async fn find_permissions_by_permit_id(
        pool: &PgPool,
        permit_id: Uuid,
    ) -> Result<Vec<String>, sqlx::Error> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
        SELECT p.name
        FROM permissions p
        INNER JOIN permit_permissions pp ON pp.permission_id = p.id
        WHERE pp.permit_id = $1
        "#,
        )
        .bind(permit_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(name,)| name).collect())
    }
}
