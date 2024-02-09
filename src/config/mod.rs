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
                jwt_secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_e| "dontusedefaultkeys".to_string()),
                thread_count: match std::env::var("THREAD_COUNT") {
                    Ok(count) => match count.parse::<usize>() {
                        Ok(count) => Some(count),
                        Err(_) => None,
                    },
                    Err(_) => None,
                },
                hash_salt: std::env::var("HASH_SALT")
                    .unwrap_or_else(|_e| "dontusedefaultsalt".to_string()),
            },
        }
    })
}
