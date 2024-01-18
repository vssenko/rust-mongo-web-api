use crate::config;
use mongodb::{Client, Database};

pub type DbError = mongodb::error::Error;

pub async fn connect() -> Database {
    let config = config::get();

    let client = Client::with_uri_str(&config.mongodb.url).await.unwrap();

    println!(
        "db: connected. Setting database name as \"{}\"",
        config.mongodb.db_name
    );

    client.database(&config.mongodb.db_name)
}
