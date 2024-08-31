pub mod model;
pub mod database;
pub mod controller;
pub mod routes;
pub mod logging;
pub mod middleware;
use database::mongo::MongoDB;
use routes::routes::app_router;
pub mod config;
//use tokio::net::TcpListener;
use config::Config;


#[tokio::main]
async fn main() {
    
    logging::init_logging();

    let config = Config::from_file("config/config.yaml");

    let db = MongoDB::init_db(&config).await;

    let app = app_router().with_state(db.into());


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on 0.0.0.0:8000 {:?} ",listener);
    axum::serve(listener, app).await.unwrap();

}
