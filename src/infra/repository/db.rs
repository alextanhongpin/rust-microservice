use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

// Alias the type for sqlx::PgPool, so that we can replace them later.
pub type DbPool = PgPool;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
