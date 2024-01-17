use dotenv;
use std::sync::OnceLock;

mod config_struct;
use config_struct::*;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[allow(unused)]
pub fn get() -> &'static Config {
    CONFIG.get_or_init(|| {
        dotenv::dotenv().ok();

        Config {
            mongodb: MongoConfig {
                url: std::env::var("MONGODB_URI")
                    .unwrap_or("mongodb://localhost:27017".to_string()),
                db_name: std::env::var("MONGODB_NAME").unwrap_or("rust-mongo-web-api".to_string()),
            },
            api: ApiConfig {
                port: match std::env::var("PORT") {
                    Ok(port) => port.parse::<u16>().unwrap(),
                    Err(_) => 3000,
                },
            },
        }
    })
}
