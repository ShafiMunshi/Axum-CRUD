use axum::{http::Method, routing::{delete, get, post}, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::upazila_handlers;

pub fn upazila_routes() -> Router {
    let cors = CorsLayer::new()// this applies the http-CORS middleware which add header 
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])// method those we need to access CRUD -- in the router middleware
        .allow_origin(Any);// allow all the request 

    let router = Router::new()
    .route("/upazila", post(upazila_handlers::create_updazila))// create upazila 
    .route("/upazila", get(upazila_handlers::get_all_upazila))// get all upazila list
    // .route("/upazila", delete(upazila_handlers::delete_upazila))// delete upazila by hardcoded id
        .route("/update_upazila", post(upazila_handlers::update_upazila))// update upazila by hardcoded id
        .route("/update_upazila/:id", post(upazila_handlers::update_from_updazila))// update upazaila using path id 
        .route("/update_upazila_from_query/:id", post(upazila_handlers::update_specifik_data_using_query))// update data of specifik id from specik table ( Using Query parameter)
        .route("/upazila/:id", get(upazila_handlers::get_from_upazila))// get specifik upazila using path id 
        .route("/upazila/:id", delete(upazila_handlers::delete_from_upazila))// delete upazila using path id
        .route("/delete_upazila/:id", delete(upazila_handlers::delete_data_using_query))// delete data of specifik id from specik table ( Using Query parameter)
        .route("/show/:id", delete(upazila_handlers::show_path))
        .layer(cors);

    router
}
