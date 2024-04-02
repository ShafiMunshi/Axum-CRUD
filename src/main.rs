use std::sync::Arc;

use axum::{
    Extension, Router,
};

use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

mod models;
mod routes;
mod handlers;
mod utils;


#[tokio::main]
async fn main() -> surrealdb::Result<()> {
      // save all the trace into a log file
      let info_file = rolling::daily("./logs", "info").with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
    .with_target(false) // target source method won't include in the log message
    .compact()// get a structured minimalist log message
    // .pretty()
    // .json()// can output json format
    // .with_writer(info_file)// save all the trace in the log file
    // .with_max_level(tracing::Level::TRACE) // set's all the details of tracing
    // .with_ansi(false)
    .init(); // initialize the tracing


    let db_instance = Arc::new(utils::db_instance::unified_db_instance().await);
    // create a db instance that can pass to any single threaded asyncronous function

    let app = Router::new()
        .merge(routes::upazila_route::upazila_routes())
        .layer(Extension(db_instance))
        .layer(TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let listerner = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // create a server at port 3000 and connect to it
    tracing::debug!("Started server at port 3000");

    axum::serve(listerner, app.into_make_service())
        .await
        .expect("App servicing failed");

    Ok(())
}
