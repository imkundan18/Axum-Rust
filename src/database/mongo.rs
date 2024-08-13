use mongodb::{Client, Collection};
use crate::model::model::Todo;
use std::sync::Arc;

pub struct MongoDB {
    pub col: Collection<Todo>,
}

impl MongoDB {
    pub async fn init_db() -> Arc<Self> {
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let database = client.database("Axum_DB");
        let col = database.collection::<Todo>("todos");
        Arc::new(MongoDB { col })
    }
}
