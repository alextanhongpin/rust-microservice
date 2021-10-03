use anyhow::Result;
use async_trait::async_trait;

use super::Repository;
use crate::domain::user::User;

pub struct UserRepository {}

// This does not work:  expected opaque type, found mutable reference.
//trait Conn<'a> = impl sqlx::PgExecutor<'a>;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository{}
    }
}

#[async_trait]
impl Repository for UserRepository {
    type Entity = User;

    async fn insert<'a, T>(&mut self, conn: T) -> Result<Self::Entity> where T: sqlx::PgExecutor<'a>{
        let user: User = sqlx::query_as!(User, "
            INSERT INTO users (name, age) VALUES ('jane', 10)
            RETURNING *
        ").fetch_one(conn).await?;

        Ok(user)
    }

    async fn find<'a, T>(&mut self, conn: T, id: &str) -> Result<Self::Entity> where T: sqlx::PgExecutor<'a>{
        let id = id.parse::<i32>()?;
        let user: User = sqlx::query_as!(User, "
            SELECT * 
            FROM users 
            WHERE id = $1
        ", id)
          .fetch_one(conn).await?;

        Ok(user)
    }

    async fn find_all<'a, T>(&mut self, conn: T) -> Result<Vec<User>> where T: sqlx::PgExecutor<'a>{
        let users: Vec<User> = sqlx::query_as!(User, "
            SELECT * 
            FROM users 
        ").fetch_all(conn).await?;

        Ok(users)
    }
}

