//use sqlx::Pool;
use sqlx::postgres::Postgres;
use sqlx::postgres::{PgPoolOptions};
use sqlx::Transaction;
use warp::Filter;
use std::env;
use anyhow::Result;

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

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    println!("starting server at port *:3030. press ctrl + c to cancel");
    warp::serve(hello)
        .run(([127,0,0,1], 3030))
        .await;

    Ok(())
}

//https://stackoverflow.com/questions/65370752/how-do-i-create-an-actix-web-server-that-accepts-both-sqlx-database-pools-and-tr
async fn insert_user<'a, E>(tx: E) -> Result<User> where E: sqlx::Executor<'a, Database = sqlx::Postgres> {
//async fn get_user_tx<'a>(tx: &'a mut Transaction<'_, Postgres>) -> Result<User> {
    // DATABASE_URL must be set to use query_as! macro.
    let user: User = sqlx::query_as!(User, "
        INSERT INTO users (name, age) VALUES ('jane', 10)
        RETURNING *
    ").fetch_one(tx).await?;

    Ok(user)
}

async fn get_user<'a, E>(pool: E) -> Result<User> where E: sqlx::Executor<'a, Database = sqlx::Postgres> {
//async fn get_user<'a>(pool: &'a Pool<Postgres>) -> Result<User> {
    // DATABASE_URL must be set to use query_as! macro.
    let user: User = sqlx::query_as!(User, "
        SELECT * 
        FROM users 
        LIMIT 1
    ").fetch_one(pool).await?;

    Ok(user)
}

async fn get_users<'a, E>(pool: E) -> Result<Vec<User>> where E: sqlx::Executor<'a, Database = sqlx::Postgres> {
    let users: Vec<User> = sqlx::query_as!(User, "
        SELECT * 
        FROM users 
    ").fetch_all(pool).await?;

    Ok(users)
}
