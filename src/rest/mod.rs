use warp::{http::{StatusCode}, Reply, Rejection};
use crate::infra::repository::{db::{DbPool}, Repository,UserRepository};
use std::convert::Infallible;
use warp::{Filter};

pub async fn user_handler(id: String, db_pool: DbPool) -> std::result::Result<impl Reply, Rejection> {
    let mut repo = UserRepository::new();
    let user = repo.find(&db_pool, &id).await;
    let json = if let Ok(u) = user {
        warp::reply::json(&u)
    } else {
        let num = vec![1,2,3,4];
        warp::reply::json(&num)
    };

    Ok(warp::reply::with_status(json, StatusCode::OK))
}

pub fn with_db(pool: DbPool) -> impl Filter<Extract = (DbPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
