use std::sync::Arc;

use axum::{
    Extension, Router,
};

use tokio::net::TcpListener;

mod models;
mod routes;
mod handlers;
mod utils;


#[tokio::main]
async fn main() -> surrealdb::Result<()> {


    let db_instance = Arc::new(utils::db_instance::unified_db_instance().await);
    // create a db instance that can pass to any single threaded asyncronous function

    let app = Router::new()
        .merge(routes::upazila_route::upazila_routes())
        .layer(Extension(db_instance));

    let listerner = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // create a server at port 3000 and connect to it
    dbg!("Started server at port 3000");

    axum::serve(listerner, app.into_make_service())
        .await
        .expect("App servicing failed");

    Ok(())
}
