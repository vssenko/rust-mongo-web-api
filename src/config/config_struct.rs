#[derive(Debug)]

pub struct MongoConfig {
    pub url: String,
    pub db_name: String,
}

#[derive(Debug)]
pub struct ApiConfig {
    pub port: u16,
    pub jwt_secret: String,
}

#[derive(Debug)]
pub struct Config {
    pub mongodb: MongoConfig,
    pub api: ApiConfig,
}
