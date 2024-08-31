use mongodb::{Client, Collection};
use crate::model::model::Todo;
use crate::config::Config;
use std::sync::Arc;

pub struct MongoDB {
    pub col: Collection<Todo>,
}

impl MongoDB {
    pub async fn init_db(config:&Config) -> Arc<Self> {
        let client = Client::with_uri_str(&config.mongodb.uri).await.unwrap();
        let database = client.database(&config.mongodb.database);
        let col = database.collection::<Todo>(&config.mongodb.collection);
        Arc::new(MongoDB { col })
    }
}
