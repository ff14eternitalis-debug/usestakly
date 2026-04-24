use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    ensure_optional_extensions(&pool).await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}

async fn ensure_optional_extensions(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF EXISTS (SELECT 1 FROM pg_available_extensions WHERE name = 'vector') THEN
                CREATE EXTENSION IF NOT EXISTS "vector";
            END IF;
        END $$;
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
