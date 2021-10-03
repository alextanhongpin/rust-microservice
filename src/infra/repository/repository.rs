use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    type Entity;

    async fn insert<'a, T>(&mut self, conn: T) -> Result<Self::Entity> where T: sqlx::PgExecutor<'a>;
    async fn find<'a, T>(&mut self, conn: T, id: &str) -> Result<Self::Entity>where T: sqlx::PgExecutor<'a>;
    async fn find_all<'a, T>(&mut self, conn: T) -> Result<Vec<Self::Entity>>where T: sqlx::PgExecutor<'a>;
}


/*
    // Testing out repository implementation.
    let mut repo = UserRepository::new();
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;
    let user = repo.insert(&mut tx).await?;
    println!("insert {:?}", user);

    let user = repo.find(&mut tx).await?;
    println!("query {:?}", user);
    tx.commit().await?;
 * */
