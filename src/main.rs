use sqlx::postgres::PgPoolOptions;
use warp::Filter;
use chrono;
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

    // DATABASE_URL must be set to use query_as! macro.
    let user: User = sqlx::query_as!(User, "
        SELECT * 
        FROM users 
        LIMIT 1
    ").fetch_one(&pool).await?;

    println!("got {:?}", user);

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127,0,0,1], 3030))
        .await;

    Ok(())
}
