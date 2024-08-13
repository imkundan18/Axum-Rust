pub mod model;
pub mod database;
pub mod controller;
pub mod routes;
use database::mongo::MongoDB;
use routes::routes::app_router;



#[tokio::main]
async fn main() {
    

    let db = MongoDB::init_db().await;

    let app = app_router().with_state(db.into());


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on 0.0.0.0:8000 {:?} ",listener);
    axum::serve(listener, app).await.unwrap();

}
