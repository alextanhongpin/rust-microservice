use sqlx::postgres::PgPoolOptions;
use chrono;

#[derive(Debug)]
struct Admin {
    id: i32,
    name: String,
    email: String,
    token: Option<String>,
    hash: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@localhost/kakuna-development").await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    println!("got {}", row.0);
    assert_eq!(row.0, 150);

    // DATABASE_URL must be set to use query_as! macro.
    let admin: Admin = sqlx::query_as!(Admin, "SELECT * FROM admins LIMIT 1")
        .fetch_one(&pool).await?;

    println!("got {:?}", admin);

    Ok(())
}
