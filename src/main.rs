use std::{ sync::Arc};

use axum::{
    extract::{Path, Query},
    routing::{delete, get, post},
    Extension, Json, Router,
};

use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    
    Surreal,
};
use tokio::net::TcpListener;

use crate::models::{record_model::Record, upazila_model::{Upazila,UpazilaId,Update_Population,}};

mod models;


type DB = Extension<Arc<Surreal<Client>>>;

async fn create_new_db() -> Surreal<Client> {
    let db = Surreal::new::<Wss>("generalpione.preciqprojects.com")
        .await
        .expect("Error : Unable to connect with Client ");

    db.signin(Root {
        username: "root",
        password: "test12345",
    })
    .await
    .expect("Error : Unable to Login");

    db.use_ns("bd")
        .use_db("bd")
        .await
        .expect("Unable to connect specified Namespace/Database");
    db
}

async fn create_updazila(Extension(db_instance): DB, Json(upazila): Json<Upazila>) -> String {
    let create_upazila: Vec<Record> = db_instance
        .create("upazila")
        .content(upazila)
        .await
        .expect("Unable to create upazila");

    format!("Added {:#?}", create_upazila)
}

// update using hardcoded id
async fn update_upazila(db_intance: DB, Json(update_people): Json<Update_Population>) -> String {
    let update_upazila_population: Option<Record> = db_intance
        .update(("upazila", "unzj02g4zmy3qn4706hu"))
        .merge(update_people)
        .await
        .unwrap();

    format!("Updated: {:#?}", update_upazila_population)
}

// update using path extractor, 
//must be serial order-> in function parameter ( DB_instance, Path_ID, Json<updateData>)
async fn update_from_updazila(db_instance: DB, Path(upazila_id): Path<String>,Json(update_people):Json<Update_Population>) -> String {
    println!("updating upazila id : {}",upazila_id);
    let upadate_distinct_upazila: Option<Record> = db_instance
        .update(("upazila", upazila_id))
        .merge(update_people)
        .await
        .expect("Something went wrong to update data");

    format!("Updated: {:#?}", upadate_distinct_upazila)
}


// update data of specifik id from specik table ( Using Query parameter)
async fn update_specifik_data_using_query(db_instance: DB,Path(table_name):Path<String>,Query(upazila_id):Query<UpazilaId>,Json(update_people): Json<Update_Population>)->String{
    let updata_query_data:Option<Record>= db_instance.update((table_name, upazila_id.id)).merge(update_people).await.expect("Something went wrong to update")  ;

    format!("Updated: {:#?}",updata_query_data)
}


// get all upazila from the upazila table
async fn get_all_upazila(db_instance: DB) -> String {
    let all_upazila: Vec<Record> = db_instance.select("upazila").await.unwrap();

    format!("List Upazila: {:#?}", all_upazila)
}

// get all upazila of specifik id using path extractor
async fn get_from_upazila(db_instance: DB, Path(upazila_id): Path<String>) -> String {
    let single_upazila: Option<Record> = db_instance
        .select(("upazila", upazila_id))
        .await
        .expect("Something went wrong to get a single database ");

    format!("Upazila: {:#?}", single_upazila)
}

// delete using specifik hardcoded id
async fn delete_upazila(db_intance: DB) -> String {
    let delete_upazila: Option<Record> = db_intance
        .delete(("upazila", ""))
        .await
        .expect("Something went wrong to delete");

    format!("Deleted {:#?}", delete_upazila)
}


// delete using path extractor
async fn delete_from_upazila(db_instance: DB, Path(upazila_id): Path<String>) -> String {
    let upadate_distinct_upazila: Option<Record> = db_instance
        .delete(("upazila", upazila_id))
        .await
        .expect("Something went wrong to update data");

    format!("Deleted: {:#?}", upadate_distinct_upazila)
}


// delete data of specifik id from specik table ( Using Query parameter)
async fn delete_data_using_query(db_instance: DB, Path(table_name):Path<String>,Query(upazila_id):Query<UpazilaId>)->String{
    println!("Deleting: table_name: {} id:{}",table_name, upazila_id.id);

    let delete_from_query:Option<Record>=db_instance.delete((table_name,upazila_id.id)).await.expect("something went wrong to delete") ;

    format!("Deleted: {:#?}",delete_from_query)
}


async fn show_path(db_instance: DB, Path(table_name):Path<String>,Query(upazila_id):Query<UpazilaId>)->String{
    let delelted:Option<Record>= db_instance.delete((table_name,upazila_id.id)).await.unwrap();
    format!("table_ ---- {:?}", delelted)

}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {

    // initilizing Tracing 
    


    let one_db_instance = Arc::new(create_new_db().await);
    // create a db instance that can pass to any single threaded asyncronous function

    let app = Router::new()
        .route("/upazila", post(create_updazila))// create upazila 
        .route("/update_upazila", post(update_upazila))// update upazila by hardcoded id
        .route("/update_upazila/:id", post(update_from_updazila))// update upazaila using path id 
        .route("/update_upazila_from_query/:id", post(update_specifik_data_using_query))// update data of specifik id from specik table ( Using Query parameter)
        .route("/upazila", get(get_all_upazila))// get all upazila list
        .route("/upazila/:id", get(get_from_upazila))// get specifik upazila using path id 
        .route("/upazila", delete(delete_upazila))// delete upazila by hardcoded id
        .route("/upazila/:id", delete(delete_from_upazila))// delete upazila using path id
        .route("/delete_upazila/:id", delete(delete_data_using_query))// delete data of specifik id from specik table ( Using Query parameter)
        .route("/show/:id", delete(show_path))
        .layer(Extension(one_db_instance));

    let listerner = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // create a server at port 3000 and connect to it
    dbg!("Started server at port 3000");

    axum::serve(listerner, app.into_make_service())
        .await
        .expect("App servicing failed");

    Ok(())
}
