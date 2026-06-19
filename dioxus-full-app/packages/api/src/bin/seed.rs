use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let permissions = vec![
        "ACCESS_DASHBOARD",
        "MANAGE_USERS",
        "DO_COMMENTS",
        "CREATE_PRODUCT",
        "APPROVE_PRODUCT",
        "UPDATE_PRODUCT",
    ];

    for name in &permissions {
        sqlx::query(
            r#"
            INSERT INTO permissions (id, name, created_at, updated_at)
            VALUES ($1, $2, NOW(), NOW())
            ON CONFLICT (name) DO NOTHING
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(name)
        .execute(&pool)
        .await?;
    }

    let admin_id = Uuid::parse_str("909c9b35-eec3-4afe-a21d-986682659f5a").unwrap();
    let user_id = Uuid::parse_str("06542153-506c-4ab2-a276-11308e188deb").unwrap();
    let seller_id = Uuid::parse_str("9e4efa8f-1e44-47af-9ab2-e04d82e47868").unwrap();

    let permits = vec![
        (admin_id, "ADMIN"),
        (user_id, "USER"),
        (seller_id, "SELLER"),
    ];

    for (id, name) in &permits {
        sqlx::query(
            r#"
            INSERT INTO permits (id, name, created_at, updated_at)
            VALUES ($1, $2, NOW(), NOW())
            ON CONFLICT (name) DO NOTHING
            "#,
        )
        .bind(id)
        .bind(name)
        .execute(&pool)
        .await?;
    }

    let links: Vec<(Uuid, &str)> = vec![
        (admin_id, "ACCESS_DASHBOARD"),
        (admin_id, "APPROVE_PRODUCT"),
        (admin_id, "MANAGE_USERS"),
        (admin_id, "UPDATE_PRODUCT"),
        (user_id, "DO_COMMENTS"),
        (seller_id, "CREATE_PRODUCT"),
        (seller_id, "UPDATE_PRODUCT"),
    ];

    for (permit_id, permission_name) in &links {
        sqlx::query(
            r#"
            INSERT INTO permit_permissions (permit_id, permission_id)
            SELECT $1, id FROM permissions WHERE name = $2
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(permit_id)
        .bind(permission_name)
        .execute(&pool)
        .await?;
    }

    Ok(())
}
