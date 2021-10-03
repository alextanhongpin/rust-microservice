use std::env;
use anyhow::Result;

mod infra;
mod domain;
mod rest;
use crate::infra::repository::{db::{create_pool}};
use crate::rest::{user_handler, with_db};

// Trait is required for warp `.and`.
use warp::{Filter};

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = create_pool(&database_url).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    println!("got {}", row.0);
    assert_eq!(row.0, 150);

    let hello_route = warp::path!("users" / String)
        .and(with_db(pool.clone()))
        .and_then(user_handler);

    let routes = hello_route.with(warp::cors().allow_any_origin());

    println!("starting server at port *:3030. press ctrl + c to cancel");
    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;

    Ok(())
}

