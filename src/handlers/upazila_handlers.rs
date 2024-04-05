use std:: sync::Arc;

use axum::{
    extract::{Path, Query}, http::StatusCode, Extension, Json
};

use surrealdb::{
    engine::remote::ws::Client,
    Surreal,
};

use crate::{models::{record_model::Record, upazila_model::{Upazila, UpazilaId, Update_Population}}, utils::api_error::ApiError};
type DB = Extension<Arc<Surreal<Client>>>;



pub async fn create_updazila(Extension(db_instance): DB, Json(upazila): Json<Upazila>) -> Result<String, ApiError> {
    let create_upazila: Vec<Record> = db_instance
        .create("upazila")
        .content(upazila)
        .await
        // .expect("Unable to create upazila");
        .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;
        //.ok_or(ApiError{message:"Not Found".to_owned(), status_code:StatusCode::NOT_FOUND, error_code:Some(30)})?;

    Ok(format!("Added {:#?}", create_upazila))
}

// update using hardcoded id
pub async fn update_upazila(db_intance: DB, Json(update_people): Json<Update_Population>) -> Result<String,ApiError> {
    let update_upazila_population: Option<Record> = db_intance
        .update(("upazila", "unzj02g4zmy3qn4706huuu")) // i added a wrong id to update data, so that it reutrn an error.
        .merge(update_people)
        .await
        .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;    
        // .unwrap();     // using unwrap will cause tokio runtime panic if error happens

    Ok(format!("Updated: {:#?}", update_upazila_population))
}

// update using path extractor, 
//must be serial order-> in function parameter ( DB_instance, Path_ID, Json<updateData>)
pub async fn update_from_updazila(db_instance: DB, Path(upazila_id): Path<String>,Json(update_people):Json<Update_Population>) -> Result<String, ApiError> {
    println!("updating upazila id : {}",upazila_id);
    let upadate_distinct_upazila: Option<Record> = db_instance
        .update(("upazila", upazila_id))
        .merge(update_people)
        .await
        .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;     
        // .expect("Something went wrong to update data");

   Ok( format!("Updated: {:#?}", upadate_distinct_upazila))
}


// update data of specifik id from specik table ( Using Query parameter)
pub async fn update_specifik_data_using_query(
    db_instance: DB,Path(table_name):Path<String>,
    Query(upazila_id):Query<UpazilaId>,
    Json(update_people): Json<Update_Population>)->Result<String, ApiError>{

    let updata_query_data:Option<Record>= db_instance
    .update((table_name, upazila_id.id))
    .merge(update_people)
    .await
    .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;     
    // .expect("Something went wrong to update")  ;

   Ok( format!("Updated: {:#?}",updata_query_data))
}


// get all upazila from the upazila table
pub async fn get_all_upazila(db_instance: DB) -> Result<String,ApiError> {
    let all_upazila:Vec<Record> = db_instance.select("upazila").await
    .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;  
    println!("all upazila list {:?}",all_upazila);

   Ok( format!("List Upazila: {:#?}", all_upazila))
}

// get all upazila of specifik id using path extractor
pub async fn get_from_upazila(db_instance: DB, Path(upazila_id): Path<String>) ->Result <String,ApiError> {
    let single_upazila: Option<Record> = db_instance
        .select(("upazila", upazila_id))
        .await
        .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;     
        // .expect("Something went wrong to get a single database ");

  Ok(  format!("Upazila: {:#?}", single_upazila))
}

// delete using specifik hardcoded id
pub async fn delete_upazila(db_intance: DB) -> Result<String, ApiError> {
    let delete_upazila: Option<Record> = db_intance
        .delete(("upazila", ""))
        .await
        .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;     
        // .expect("Something went wrong to delete");

    Ok(format!("Deleted {:#?}", delete_upazila))
}


// delete using path extractor
pub async fn delete_from_upazila(db_instance: DB, Path(upazila_id): Path<String>) -> Result<String , ApiError> {
    let upadate_distinct_upazila: Option<Record> = db_instance
        .delete(("upazila", upazila_id))
        .await
        .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;     
        // .expect("Something went wrong to update data");

    Ok(format!("Deleted: {:#?}", upadate_distinct_upazila))
}


// delete data of specifik id from specik table ( Using Query parameter)
pub async fn delete_data_using_query(db_instance: DB, Path(table_name):Path<String>,Query(upazila_id):Query<UpazilaId>)->Result<String,ApiError>{
    println!("Deleting: table_name: {} id:{}",table_name, upazila_id.id);

    let delete_from_query:Option<Record>=db_instance
    .delete((table_name,upazila_id.id))
    .await
    .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;     
    // .expect("something went wrong to delete") ;

    Ok(format!("Deleted: {:#?}",delete_from_query))
}


pub async fn show_path(db_instance: DB, Path(table_name):Path<String>,Query(upazila_id):Query<UpazilaId>)->Result<String, ApiError>{
    let delelted:Option<Record>= db_instance
    .delete((table_name,upazila_id.id))
    .await
    .map_err(|error|ApiError{message:error.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR,error_code: Some(10)})?;     
    // .unwrap();
    Ok(format!("table_ ---- {:?}", delelted))

}




