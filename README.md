# rust-mongo-web-api

Rust | Actix Web | MongoDb | Integration tests | Node Test Runner | In-memory MongoDb.

## Introduction

When I got some free time after years of workload, I made a bit of reflection to understand where I am and what I want. Last years were spent on different modern startups, usually with a blend of NodeJS, AWS, MongoDb and React/Flutter. But seasons changes, and I am not getting younger. With rememberance of university education and my passion of old-school development, I've decided to switch my career to Rust development.

This project, being a first transition step, is an analysis of Actix web framework and ways to enhance development with it.

## Part one: Actix web

Actix web seems to be the most popular Rust framework for building web APIs. You can check more on their [official website](https://actix.rs/).

### Handlers

It is a framework, which basically allow you to write functions as handlers, and integrate them into App. To make it well structured, let's split application into routes files, which will be attached to application on start. Each route file is located in `/src/routes` and exports `pub fn scope() -> Scope` function.

Example of simplest /status routes setup and use:

```
// src/routes/status.rs

use actix_web::{get, web, HttpResponse, Responder, Scope};
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    status: String,
}

#[get("")]
async fn get_status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

pub fn scope() -> Scope {
    let scope = web::scope("/status").service(get_status);

    scope
}
```

All routes can and should be grouped as one module:

```
// src/routes/mod.rs
pub mod status;
```

And then applied into main app:

```
// src/main.rs

//other imports
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(routes::status::scope())
        )
        .bind(("127.0.0.1", config::get().api.port))?
        .run()
        .await
}
```

Each `Scope` is applied to App with `.service()` method.

### Config

All configuration variables (such port, mongourl and other variables passed from .env) is better to be stored in one place.
Coming from NodeJS, where configuration is usually exported as single javascript object (and being a singletone just by exporting), I thought it will be good to create clear nested configuration structure:

```
// src/config/config_struct.rc

#[derive(Debug)]
pub struct MongoConfig {
    pub url: String,
    pub db_name: String,
}

#[derive(Debug)]
pub struct ApiConfig {
    pub port: u16,
    pub jwt_secret: String,
    pub thread_count: Option<usize>,
    pub hash_salt: String,
}

#[derive(Debug)]
pub struct Config {
    pub mongodb: MongoConfig,
    pub api: ApiConfig,
}
```

To make it easy to retrieve configuration from any part of application, I decided to make it as publically accessible singleton.
Thankfully, Rust has great mechanism to achieve that: `std::sync::OnceLock`.

```
// src/config.mod.rs

use dotenv;
use std::sync::OnceLock;

mod config_struct;
use config_struct::*;

// here we describe static variable holding our configuration
static CONFIG: OnceLock<Config> = OnceLock::new();


// here we have public function to get our configuration value.
// it will execute get_or_init method on OnceLock variable.
pub fn get() -> &'static Config {

    // here we will return config if value is already initialized, otherwise we will init it and return.
    CONFIG.get_or_init(|| {
        // load .env variables
        dotenv::dotenv().ok();

        // return config structure
        Config {
            mongodb: MongoConfig {
                url: std::env::var("MONGODB_URI")
                    .unwrap_or("mongodb://localhost:27017".to_string()),
                // ...other mongo config values
            },
            api: ApiConfig {
                port: match std::env::var("PORT") {
                    Ok(port) => port.parse::<u16>().unwrap(),
                    Err(_) => 3000,
                },
                // ...other mongo config values
            },
        }
    })
}
```

This approach allows us to easily get any configuration value from any place, like `crate::config::get().api.port`,

### Db Connection & Dependency Injection

//TODO: finish

### MongoDb models

//TODO: finish

### Middleware?

//TODO: finish

## Part two: Node Test Runner

//TODO: finish
