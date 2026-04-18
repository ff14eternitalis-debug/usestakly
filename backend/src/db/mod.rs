use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect_lazy(database_url)
}
