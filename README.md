# rust-mongo-web-api

Sample (and subjective) project structure for Rust web api with MongoDb connection.
Created with intention to recreate (imho) graceful structure of good-written NodeJS server.
To increase development speed and ease of test coverage, integration tests are written with Node test runner.

## Thoughts

1. actix_web seems a little bit strange and tough in its procedural design. Typical how-to-actix-web articles shows how to pass some database connection in http handlers, which looks way upside down. To brush up the structure i've decided to use some dependency container (which i called injector, which is not an injector but a container). Passing AppState struct to handlers and getting appropriate service from conteiner (like state.i.user_service()) seems way better code organization.

2. Middleware. Usually, when writing NodeJS apps, you want to add some authorization middleware, which will check headers and (optionally but usually yes) load User somewhere in request (req.user). With actix_web I found it surprisingly challeging to setup authorization middleware and adding it to specific routes(handlers). Instead, I found much easier to manually call `state.i.auth_service().get_user_from_req(&req)`.

3. Mongo driver. Generally, serde+mongodb crates gives you ability to easily create simple structs. However, the very first implementation faced an issue with ObjectId (\_id) field (de)serialization. There is mongodb serde helper to mimic plain hex value, but this helper does not support optional ids. And as a result, it writes \_id as string, which is not something great. To simplify that, i just decided to explicitly use string \_id type.

4. Config. To make it easier to get config from different parts of application, i've designed Config mod to contain singleton config value. In very secure Rust world it was surprisingly pleasant to create such antipattern as singleton using `OnceLock`.

5. With lack of actix_web graceful integration tests (the only thing in documentation I found was about recreating new App and adding some routes), I've decided to spin Rust app from Nodejs and use Nodejs with in-memory Mongo to create graceful full-featured tests.

## Conclusion

Well, of course it's not that fast app development as in NodeJS, but this is Rust! And considering Rust a low-level performant programming language, it can still relatively easy be used to write business logic, and this is awesome.
