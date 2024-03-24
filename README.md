# Axum-CRUD
This documentation is all about linking Axum with SurrealDB. 
## Topic- we covered
1. create 
```rust
async fn create_updazila(Extension(db_instance): DB, Json(upazila): Json<Upazila>) -> String {
    let create_upazila: Vec<Record> = db_instance
        .create("upazila")
        .content(upazila)
        .await
        .expect("Unable to create upazila");

    format!("Added {:#?}", create_upazila)
}
```