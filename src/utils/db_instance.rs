use surrealdb::{engine::remote::ws::{Client, Wss}, opt::auth::Root, Surreal};


pub async fn unified_db_instance() -> Surreal<Client> {
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