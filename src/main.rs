//use sqlx::Pool;
use sqlx::postgres::Postgres;
use sqlx::postgres::{PgPoolOptions};
use sqlx::Transaction;
use warp::Filter;
use std::env;
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    profile: Option<String>,
    age: Option<i32>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    println!("got {}", row.0);
    assert_eq!(row.0, 150);

    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;
    let user = insert_user(&mut tx).await?;
    println!("insert {:?}", user);

    let user = get_user(&mut tx).await?;
    println!("query {:?}", user);
    tx.commit().await?;

    let user = get_user(&pool).await?;
    println!("got user {:?}", user);

    let users = get_users(&pool).await?;
    println!("got users {:?}", users);

    // Testing out repository implementation.
    let mut repo = Repository::new();
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;
    let user = repo.insert(&mut tx).await?;
    println!("insert {:?}", user);

    let user = repo.find(&mut tx).await?;
    println!("query {:?}", user);
    tx.commit().await?;

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    println!("starting server at port *:3030. press ctrl + c to cancel");
    warp::serve(hello)
        .run(([127,0,0,1], 3030))
        .await;

    Ok(())
}

#[async_trait]
pub trait UserRepository {
    type Entity;

    async fn insert<'a, T>(&mut self, conn: T) -> Result<Self::Entity> where T: sqlx::PgExecutor<'a>;
    async fn find<'a, T>(&mut self, conn: T) -> Result<Self::Entity>where T: sqlx::PgExecutor<'a>;
    async fn find_all<'a, T>(&mut self, conn: T) -> Result<Vec<Self::Entity>>where T: sqlx::PgExecutor<'a>;
}

struct Repository {}

impl Repository {
    fn new() -> Self {
        Repository{}
    }
}

#[async_trait]
impl UserRepository for Repository {
    type Entity = User;

    async fn insert<'a, T>(&mut self, conn: T) -> Result<Self::Entity> where T: sqlx::PgExecutor<'a>{
        let user: User = sqlx::query_as!(User, "
            INSERT INTO users (name, age) VALUES ('jane', 10)
            RETURNING *
        ").fetch_one(conn).await?;

        Ok(user)
    }

    async fn find<'a, T>(&mut self, conn: T) -> Result<Self::Entity> where T: sqlx::PgExecutor<'a>{
        let user: User = sqlx::query_as!(User, "
            SELECT * 
            FROM users 
            LIMIT 1
        ").fetch_one(conn).await?;

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


//https://stackoverflow.com/questions/65370752/how-do-i-create-an-actix-web-server-that-accepts-both-sqlx-database-pools-and-tr
async fn insert_user<'a, T>(tx: T) -> Result<User> where T: sqlx::PgExecutor<'a> {
//async fn get_user_tx<'a>(tx: &'a mut Transaction<'_, Postgres>) -> Result<User> {
    // DATABASE_URL must be set to use query_as! macro.
    let user: User = sqlx::query_as!(User, "
        INSERT INTO users (name, age) VALUES ('jane', 10)
        RETURNING *
    ").fetch_one(tx).await?;

    Ok(user)
}

async fn get_user<'a, T>(pool: T) -> Result<User> where T: sqlx::PgExecutor<'a> {
    let user: User = sqlx::query_as!(User, "
        SELECT * 
        FROM users 
        LIMIT 1
    ").fetch_one(pool).await?;

    Ok(user)
}

async fn get_users<'a, T>(pool: T) -> Result<Vec<User>> where T: sqlx::PgExecutor<'a> {
    let users: Vec<User> = sqlx::query_as!(User, "
        SELECT * 
        FROM users 
    ").fetch_all(pool).await?;

    Ok(users)
}
