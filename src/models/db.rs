use crate::config;
use wither::mongodb::{Client, Database};

pub async fn connect() -> Database {
    let config = config::get();
    let client = Client::with_uri_str(&config.mongodb.url).await.unwrap();

    client.database(&config.mongodb.db_name)
}
