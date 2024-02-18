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
- [Part two: Node Test Runner(#part-two-node-test-runner)
  - [Choosing test framework](#choosing-test-framework)
  - [Tests context](#tests-context)
  - [Bootstap](#bootstap)
  - [Virtual MongoDb](#virtual-mongodb)
  - [Test API instance](#test-api-instance)
  - [Additional Testers and helpers](#additional-testers-and-helpers)
  - [Full-featured test example](#full-featured-test-example)
  - [Test preparing?](#test-preparing)
- [Conclusion](#conclusion)

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

With part one implemented we already have a system easily adjustable to mocked unit testing with dependency injections through constructor parameteres, with the only difference that mocking will require to pass dependencices as a-la interfaces with `dyn impl IService`. But my personal preferences and experience tells me to use integration testing over unit testing. With unit testing we grab some piece of code and test this isolated piece of code. With integration tests we spin up application with real or mocked resources and test the entire flow.

Currently (as for actix_web version 4), actix_web framework does not allow you to create a method returing App instance (framework does not have `AppEntry` as exported type). With unability to recreate our entire application out of main function, we do not have ability to build our App in Rust integration tests. And indeed, current Actix web guide [advise us](https://actix.rs/docs/testing/) to create separated `App` instance in tests, and attach needed handlers/scopes/app_data. This is far from normal integration testing, as we do not actually tests **our** application, rather than **another** application with parts of our code.

With said above, an integration testing flow requires spinning entire app (with `cargo run` or directly with executing compiled program).

But if we already spin up Rust application without any Rust context, why not to choose the most flexible and ease-to-go solution for integration tests?

### Choosing test framework

While we have a lot of go-to options, I've chosen NodeJS (well, no surprises, I've spent 7 years with it).

Writing tests is always seems as "unnecessary" work, and our goal is to speed and simplify writing tests as much as possible, and NodeJS with interpreted language Javascript is one of the fastests solutions in terms of development speed.

I was also thinking about Typescript vs Javascript for testing, and found that Typescript, while being almost a must for large projects, is far less useful for tests itself, because almost all communication with application is done through HTTP calls with obviously dynamic JSON responses. Another point to Javascript is that NodeJS natively supports only Javascript, and for me, personally, this still matters. For native Typescript support we could use Deno as (long-long) coming replacement of NodeJS, but our tests involves in-memory MongoDB npm package, and unfortunately Deno does not have 100% compatibility with npm packages.

The last, but not least point to NodeJS is that both JS/TS use the same mustashe bracket {} style as Rust itself (i'm referencing to Python-like languages in this sentence, of course).

As for test framework itself, candidates were `mocha`, `ava` and `NodeJS test runner`. Main and most important technical difference is that Mocha is running tests in one process, while Ava is using separate process per test file. in terms of concurrent speed running tests in parallel processes is always more performant, but this requires specific tests setup. Nice part of our testing approach is that we spin up **in-memory** MongoDB instance instead of using real one. With help of that, we can easily spin up separated in-memory MongoDB instance for each test process. To clarify, if we would use real single MongoDB database instance, we would either write tests always remembering that there is always something more in database from other tests, or just run tests sequentially, one-by-one.

As a result, I would use Ava as main test runner, but thankfully NodeJS team implemented it's own test runner, which run tests in the same one-file-one-process flow as Ava itself! Remembering that the more native the better, let's stick with NodeJS Test runner with Javascript. We also have the most laconic test command, `node --test`.

### Tests context

To help developers write tests in the fastest possible way, it's good to have an entire test infrastructure.
What our API should already have?

- Running MongoDB.
- Running API pointing to that MongoDB.
- Easy way to get axios instance pointing to that API.
- Support of authorized/non-authorized requests.
- Some test methods to easily apply generic test cases, like required athorization for endpoint tests.
- Some helper methods to easily create/manage application entities, like users.

Let's wrap it all in one file:

```
//tests/_context/index.js

import { getApi } from "./api.js";
import { bootstrap, shutdown } from "./bootstrap.js";
import user from "./helpers/user.js";
import test from "./addtionalTesters.js";

export default {
  api: getApi,
  bootstrap,
  shutdown,
  user,
  test,
};

```

### Bootstap

In most test files, it would be easy to spin both MongoDB and API with one command before tests, and shutdown them after tests.

Obviously, this functions look just as you expect:

```
//tests/_context/bootstrap.js
//other code
export async function bootstrap() {
  await mongo.createMongo();
  await api.startApi({
    mongourl: mongo.getUrl(),
    port: await _getFreePort(),
  });
}

export async function shutdown() {
  await api.stopApi();
  await mongo.stopMongo();
}

```

In test files itself, it's dramatically easy to use our environment. For example, /status route tests:

```
//tests/routes/status.test.js
import test from "node:test";
import assert from "node:assert";
import context from "../_context/index.js";

test.describe("/status", () => {
  test.before(async (t) => await context.bootstrap());
  test.after(async (t) => await context.shutdown());

  test.it("should return status ok", async () => {
    const result = await context.api().get("/status");
    assert.deepEqual(result.data, {
      status: "Ok",
    });
  });
});
```

Here we `bootstrap` our API on `test.before`, `shutdown` our API on `test.after`, and use `context.api()` to retrieve `axios` instance for testing. For assertion we use native node:assert, but you can always swap it for Chai.

### Virtual MongoDb

With the fact that we have test file (test group if you want) per process, we can easily make our mongo.js file as module containing and managing mongodb connection inside itself:

```
//tests/_context/mongo.js

import { MongoClient, Db } from "mongodb";
import { MongoMemoryServer } from "mongodb-memory-server";

let mongoServer = null;
let mongourl = null;
let mongoClientPromise = null;

export async function createMongo() {
  mongoServer = await MongoMemoryServer.create();
  mongourl = mongoServer.getUri();

  console.log(`mongo: created with url "${mongourl}"`);
}

export async function stopMongo() {
  if (mongoClientPromise) {
    const mongoClient = await mongoClientPromise;
    await mongoClient.close();
  }
  await mongoServer.stop();
  mongoServer = null;
  mongourl = null;

  console.log("mongo: stopped");
}

export function getUrl() {
  if (!mongourl) throw new Error("Mongo server is not started");
  return mongourl;
}

//getDatabase method

export default {
  createMongo,
  stopMongo,
  getUrl,
  getDatabase,
};
```

### Test API instance

With the same idea, we can wrap starting and stopping API in separated file:

```

import _ from "lodash";
import axios from "axios";
import { spawn, shutdownProcess } from "./spawn.js";

const secondsToWaitAfterStart = 3;

let apiProcess = null;
let serverUrl = null;

//AxiosSimpleError class definition

export async function startApi({ mongourl, port }) {
  apiProcess = await spawn({
    command: `cargo run --quiet`,
    args: [],
    options: {
      cwd: process.cwd(),
      shell: true,
      env: {
        MONGODB_URI: mongourl,
        PORT: port,
        THREAD_COUNT: "2",
      },
    },
    waitForOutput: "db: connected.",
  });

  await new Promise((r) => setTimeout(r, secondsToWaitAfterStart * 1000));

  serverUrl = `http://127.0.0.1:${port}`;

  console.log(`api: started with url "${serverUrl}"`);

  return {
    serverUrl,
  };
}

export async function stopApi() {
  if (!apiProcess) return;
  shutdownProcess(apiProcess);
}

export function getApi({ token, simplifyErrors = true } = {}) {
  const headers = {};

  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const axiosInstance = axios.create({
    baseURL: serverUrl,
    headers,
  });

  if (simplifyErrors) {
    axiosInstance.interceptors.response.use(
      (response) => response,
      (error) => {
        return Promise.reject(new AxiosSimpleError(error));
      },
    );
  }

  return axiosInstance;
}

export default {
  startApi,
  stopApi,
  getApi,
};
```

To make work with cmd from NodeJS easier, I've created some utils for that. On spawning process we want to pipe all process output to our own console. It is also cool to be able to wait for specific output from spawned process and resolve Promise only after that. On process termination, I found that Windows system requires all process streams to be manually destroyed, otherwise it will not end the process. For more details of spawning and terminating processes from NodeJS, check `/tests/_context/spawn.js`.

Inside API file it is also handy to write axios builder function, which points to created API and can have additional arguments.

### Additional Testers and helpers

Check `/tests/_context/helpers` and `/tests/_context/additionalTesters.js` files for more details, it's pretty straightforward.

### Full-featured test example

With that system, we not only can develop ultra-fast Rust-based Servers, with decent structure and development-scale capabilities, we also can test our API enpoints literally with few lines of code! Let's test our /posts enpoints:

- `GET /posts` will return no posts
- `POST /posts` requires authorization,
- `POST /posts` will return new post, and after that `GET /posts` will also return new post.

```
import test from "node:test";
import assert from "node:assert";
import context from "../_context/index.js";

test.describe("/posts", () => {
  let registerData;

  test.before(async (t) => await context.bootstrap());
  test.after(async (t) => await context.shutdown());

  test.before(async () => {
    registerData = await context.user.registerUser();
  });

  test.it("get /posts should return no posts", async () => {
    const result = await context.api().get("/posts");
    assert.deepEqual(result.data, []);
  });

  context.test.unauthorized({
    url: "/posts",
    method: "post",
    data: { title: "Some title", content: "Some content" },
  });

  test.it("post /posts should create post for authorized user", async () => {
    const result = await context
      .api({ token: registerData.token })
      .post("/posts", {
        title: "Some title",
        content: "Some content",
      });

    assert.ok(result.data._id);
    assert.equal(result.data.title, "Some title");
    assert.equal(result.data.user_id, registerData.user._id);

    const getAllResult = await context.api().get("/posts");
    assert.deepEqual(getAllResult.data.length, 1);
    assert.deepEqual(getAllResult.data[0]._id, result.data._id);
  });
});
```

### Test preparing?

Not for this topic, but `mongodb-memory-server` will **download** neccessary binaries on first launch. To prevent strange behaviour on running tests in parallel, there is handy `npm run test:prepare` command to warm up this module. This may come helpful in CICD test execution, like Github Actions. Tested on personal experience.

## Conclusion

In programming world, there are different tasks and different tools. Choosing the right tool for the right task and using that tool in the most appropriate way is one of the many keys to success.

Here we created literally fastest API with Rust, we confirmed that low-level Rust language is pretty expressive to write high-level business logic if needed. While Rust requires additional time for development, we minigate that expenses with the fastest testing approach with properly used Node Test Runner.
