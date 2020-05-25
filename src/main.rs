use std::sync::Arc;
use tokio_postgres::{Error, NoTls};

mod db;
mod models;

use crate::db::DbContext;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let connection_string = format!(
        "host={} user={} password={}",
        "localhost", "postgres", "example"
    );
    let (client, connection) = tokio_postgres::connect(&connection_string[..], NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("Error connection to database {}", e);
        }
    });

    let db_context = DbContext::with_client(client)
        .await
        .expect("Failed to connect to database");
    let db = Arc::new(db_context);

    warp::serve(filters::posts(db))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

mod filters {
    use warp::Filter;

    use super::db::Db;
    use super::handlers;
    use super::models;

    pub fn posts(
        db: Db,
    ) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        hello()
            .or(post_create(db.clone()))
            .or(post_list(db.clone()))
    }
    pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("hello")
            .and(warp::get())
            .and_then(handlers::hello)
    }

    pub fn post_create(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("post")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::create_post)
    }

    pub fn post_list(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("post")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::list_posts)
    }

    fn json_body() -> impl Filter<Extract = (models::Post,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use super::db::Db;
    use super::models;

    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn hello() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::json(&String::from("hello")))
    }

    pub async fn create_post(create: models::Post, db: Db) -> Result<impl warp::Reply, Infallible> {
        println!("create_post: {:?}", create);

        if let Ok(_) = db.insert_post(&create).await {
            Ok(StatusCode::CREATED)
        } else {
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

    pub async fn list_posts(db: Db) -> Result<impl warp::Reply, Infallible> {
        println!("getting all posts");

        if let Ok(posts) = db.get_all_posts().await {
            Ok(warp::reply::json(&posts))
        } else {
            Ok(warp::reply::json(&String::from("error retrieving posts")))
        }
    }
}
