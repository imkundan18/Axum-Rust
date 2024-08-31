use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use axum::extract::Path;
use bson::to_bson;
use crate::model::model::Todo;
use crate::database::mongo::MongoDB;
use futures::stream::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
//use mongodb::options::FindOptions;
use std::sync::Arc;

pub struct TodoList{}

impl TodoList {
pub async fn create_todo(State(db):State<Arc<MongoDB>>, Json(todo):Json<Todo>)->impl IntoResponse{
    let new_todo=todo;
    match db.col.insert_one(new_todo, None).await{
        Ok(res) => (StatusCode::CREATED, Json(res.inserted_id)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_all_todos(State(db): State<Arc<MongoDB>>) -> impl IntoResponse {
    let mut cursor = match db.col.find(None, None).await
     {
        Ok(cursor) => cursor,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let mut todos = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(todo) => todos.push(todo),
            Err(_) => return StatusCode::BAD_REQUEST.into_response(),
        }
    }
    (StatusCode::OK, Json(todos)).into_response()
}


pub async fn update_todo(State(db):State<Arc<MongoDB>>,
                        Path(id):Path<String>, Json(new_data): Json<Todo>,
) -> impl IntoResponse {
    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    let filter=doc! {"_id":obj_id};
    
    //let new_doc=doc! {"$set": new_data};
    let new_todo=Todo{
        id: Some(obj_id),
        title:new_data.title.to_owned(),
        description:new_data.description.to_owned(),
        completed:new_data.completed,
    };
    let b_data=to_bson(&new_todo).unwrap();
    let new_doc=doc! {"$set": b_data};
    
    //let message="Sucessfully update".to_string();
    match db.col.update_one(filter, new_doc, None).await{
        Ok(res)=>(StatusCode::ACCEPTED,Json(res.modified_count)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

    pub async fn delete_todo(State(db):State<Arc<MongoDB>>,Path(id):Path<String>)->
    impl IntoResponse{
        let obj_id=id;
        let filter=doc! {"_id":obj_id};
        match db.col.delete_one(filter, None).await{
            Ok(res)=>(StatusCode::OK,Json(res.deleted_count)).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }

}



}


// pub async fn create_todo(State(db):State<Arc<MongoDB>>, Json(todo):Json<Todo>)->impl IntoResponse{
//     let new_todo=todo;
//     match db.col.insert_one(new_todo, None).await{
//         Ok(res) => (StatusCode::CREATED, Json(res.inserted_id)).into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

// pub async fn get_all_todos(State(db): State<Arc<MongoDB>>) -> impl IntoResponse {
//     let mut cursor = match db.col.find(None, None).await
//      {
//         Ok(cursor) => cursor,
//         Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     };
//     let mut todos = Vec::new();
//     while let Some(result) = cursor.next().await {
//         match result {
//             Ok(todo) => todos.push(todo),
//             Err(_) => return StatusCode::BAD_REQUEST.into_response(),
//         }
//     }
//     (StatusCode::OK, Json(todos)).into_response()
// }


// pub async fn update_todo(State(db):State<Arc<MongoDB>>,
//                         Path(id):Path<String>, Json(new_data): Json<Todo>,
// ) -> impl IntoResponse {
//     let obj_id = match ObjectId::parse_str(&id) {
//         Ok(oid) => oid,
//         Err(_) => return StatusCode::BAD_REQUEST.into_response(),
//     };
//     let filter=doc! {"_id":obj_id};
    
//     //let new_doc=doc! {"$set": new_data};
//     let new_todo=Todo{
//         id: Some(obj_id),
//         title:new_data.title.to_owned(),
//         description:new_data.description.to_owned(),
//         completed:new_data.completed,
//     };
//     let b_data=to_bson(&new_todo).unwrap();
//     let new_doc=doc! {"$set": b_data};
    
//     //let message="Sucessfully update".to_string();
//     match db.col.update_one(filter, new_doc, None).await{
//         Ok(res)=>(StatusCode::ACCEPTED,Json(res.modified_count)).into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

//     pub async fn delete_todo(State(db):State<Arc<MongoDB>>,Path(id):Path<String>)->
//     impl IntoResponse{
//         let obj_id=id;
//         let filter=doc! {"_id":obj_id};
//         match db.col.delete_one(filter, None).await{
//             Ok(res)=>(StatusCode::OK,Json(res.deleted_count)).into_response(),
//             Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//         }

// }
