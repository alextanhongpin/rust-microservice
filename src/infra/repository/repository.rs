use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    type Entity;

    async fn insert<'a, T>(&mut self, conn: T) -> Result<Self::Entity> where T: sqlx::PgExecutor<'a>;
    async fn find<'a, T>(&mut self, conn: T) -> Result<Self::Entity>where T: sqlx::PgExecutor<'a>;
    async fn find_all<'a, T>(&mut self, conn: T) -> Result<Vec<Self::Entity>>where T: sqlx::PgExecutor<'a>;
}

