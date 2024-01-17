use actix_web::{App, HttpServer};
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::status::scope())
            .service(routes::posts::scope())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
