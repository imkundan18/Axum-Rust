
use axum::{routing::{post,get,put}, Router};
use crate::controller::controller::{create_todo,get_all_todos,update_todo};
use crate::database::mongo::MongoDB;
use std::sync::Arc;

pub fn app_router() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/todo", post(create_todo))
        .route("/todo/all", get(get_all_todos))
        .route("/todo/update/:id", put(update_todo))
}

//get_all_todos