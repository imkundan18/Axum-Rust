
use axum::{routing::{delete, get, post, put}, Router};
//use crate::controller::controller::{get_all_todos,update_todo,delete_todo};
use crate::controller::controller::TodoList;//{create_todo,get_all_todos,update_todo,delete_todo};
use crate::database::mongo::MongoDB;
use crate::middleware::log_request;
use axum::middleware::from_fn;
use std::sync::Arc;

pub fn app_router() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/todo", post(TodoList::create_todo))
        .route("/todo/all", get(TodoList::get_all_todos))
        .route("/todo/update/:id", put(TodoList::update_todo))
        .route("/delete/:id", delete(TodoList::delete_todo))
       .layer(from_fn(log_request))
    }

