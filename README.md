# rust-mongo-web-api

Rust | Actix Web | MongoDb | Integration tests | Node Test Runner | In-memory MongoDb.

## Content

- [Introduction](#introduction)
- [Part one: Actix web](#part-one-actix-web)
  - [Handlers](#handlers)
  - [Config](#config)
  - [Db Connection & Dependency Injection](#db-connection--dependency-injection)
  - [MongoDb models](#mongodb-models)
  - [Middleware?](#middleware)
- [Part two: Node Test Runner](#part-two-node-test-runner)

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

What Actix web (& Mongo driver documentation) tells us about managing db connection in Actix web server? it's to put that connection in application data, and then use it inside handlers (like [this](https://github.com/actix/examples/tree/master/databases/mongodb) example, for example). Being worried that this approach will be settled everywhere (despite the fact that this was just simpliest example, like take-this-and-make-better-from-this-point), I want to show slightly better usage of Actix web application data.

As a first step (having in mind that we may need multiple connections to different resourses, like db+reddis, or something else) let's wrap our application data in structure.

But we (usually) use our db connection to perform some business logic. And it's common approach to write business logic inside some Services classes, which get all required resources/services/etc via constructor parameters. This not only makes easier calling business logic methods, but also opens for us a way to unit test our application. (Important note: I'm not a big fan of OOP structure of application (like .NET projects) and in NodeJS always go without it (because it's (a) single thread, (b) easy to mock everything with [proxyquire](https://www.npmjs.com/package/proxyquire) or similar packages in case of unit testing))

So, as natural evolving of these two points, why not to create our own dependency injector and put it inside application data?

```
/src/app_state.rs
use crate::injector::Injector;
use actix_web::{error, HttpResponse};

#[derive(Debug)]
pub struct AppState {
    pub i: Injector,
}

impl AppState {
    pub fn format_err(&self, e: error::Error) -> HttpResponse {
        return HttpResponse::from_error(e);
    }
}
```

We are not only putting injector here, but also making it handy to put some http utils methods for ease of calling.

```
//src/injector/mod.rs

use std::rc::Rc;

use mongodb::Database;

use crate::models;
use crate::services;

pub async fn new() -> Injector {
    let db = models::db::connect().await;
    let db_rc = Rc::new(db.clone());
    let auth_service = Rc::new(services::AuthService::new());

    let user_service = Rc::new(services::UserService::new(
        Rc::clone(&db_rc),
        auth_service.clone(),
    ));
    let post_service = Rc::new(services::PostService::new(Rc::clone(&db_rc)));

    Injector {
        single_db: Rc::clone(&db_rc),
        single_auth_service: auth_service,
        single_user_service: user_service,
        single_post_service: post_service,
    }
}

// fields on injector are used to store "singleton" services
#[derive(Debug)]
pub struct Injector {
    single_db: Rc<Database>,
    single_auth_service: Rc<services::auth::AuthService>,
    single_user_service: Rc<services::user::UserService>,
    single_post_service: Rc<services::post::PostService>,
}

impl Injector {
    pub fn user_service(&'_ self) -> &'_ services::user::UserService {
        &self.single_user_service
    }

    pub fn post_service(&'_ self) -> &'_ services::post::PostService {
        &self.single_post_service
    }

    pub fn auth_service(&'_ self) -> &'_ services::auth::AuthService {
        return &self.single_auth_service;
    }
}
```

As you can see, we create instances of all resources/services on the heap via Reference counters, and pass where they needed. Also, we have handy methods for retrieving instances of needed services (though this code can create new instsance per call instead of singletons, it's up to you).

With help of this trick, our single http handler with service usage looks like that:

```
//src/routes/posts.rs

///imports...

#[get("")]
async fn get_all_posts(state: web::Data<AppState>) -> impl Responder {
    let result = state.i.post_service().list().await;

    match result {
        Ok(posts) => return HttpResponse::Ok().json(posts),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

///other handlers...
```

### MongoDb models

I found it rather easy to work with raw Rust Mongodb driver, because it already supports automatic serialization&deserialization documents into/from structures. Simple Post model looks like this:

```
//src/models/post.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub _id: String,
    pub title: String,
    pub content: String,
    pub user_id: String,
}
```

And simple example of collection usage:

```
//src/services/post.rs

// collection creation in constructor method
  let collection = db.collection::<Post>("posts");
//...

//create method
pub async fn create(
    &self,
    post_data: CreatePostData,
    user_id: &str,
) -> Result<Post, mongodb::error::Error> {
    let insert_result = self
        .collection
        .insert_one(
            Post {
                _id: ObjectId::new().to_hex(),
                title: post_data.title,
                content: post_data.content,
                user_id: user_id.to_string(),
            },
            None,
        )
        .await;

    //other creationg logic logic...
}
```

### Middleware?

Actix web has surprisingly [complicated middleware](https://actix.rs/docs/middleware/), involving third party futures libs. Not only that, it is not supposed to add middleware to specific routes (like we have User router(scope) and we want to put auth check on 3 from 5 routes). With these facts, as well as with little documentation, I've decided to omit usage of middleware at all and just use service methods for any specific logic. For example, instead of auth middleware we have `get_user_from_req` and `get_user_from_req_with_role` methods in user service. And usage of that (in my opinion) is still pretty laconic:

```
//src/services/user.rs

///imports

pub async fn get_user_from_req(&self, req: &HttpRequest) -> Result<User, errors::Error> {
    let Some(auth_header) = req.headers().get("Authorization") else {
        return Err(errors::build_unauth_err());
    };

    let Ok(auth_header) = auth_header.to_str() else {
        return Err(errors::build_unauth_err());
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(errors::build_unauth_err());
    }

    let token = auth_header.replace("Bearer ", "");
    let decoded_token = self.auth_service.decode_token(token.as_str());

    if decoded_token.is_err() {
        return Err(errors::build_unauth_err());
    }

    let decoded_token = decoded_token.unwrap();

    let user = self.get_by_id(&decoded_token.user_id).await;

    match user {
        Some(user) => return Ok(user),
        None => return Err(errors::build_unauth_err()),
    }
}

///other methods
```

```
//src/routes/users.rs

#[get("/me")]
async fn get_me(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user = state.i.user_service().get_user_from_req(&req).await;

    if user.is_err() {
        return state.format_err(user.unwrap_err());
    }

    return HttpResponse::Ok().json(user.unwrap());
}
```

## Part two: Node Test Runner

//TODO: finish
