mod app_state;
mod config;
mod injector;
mod models;
mod routes;
mod services;
mod utils;
use actix_web::{App, HttpServer};
pub use app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let workers_count = config::get().api.thread_count;

    let mut server = HttpServer::new(|| {
        App::new()
            .data_factory(|| async {
                let injector = injector::new().await;
                let app_state = app_state::AppState { i: injector };

                Ok::<_, AppState>(app_state)
            })
            .service(routes::status::scope())
            .service(routes::users::scope())
            .service(routes::posts::scope())
    });

    if workers_count.is_some() {
        server = server.workers(workers_count.unwrap());
    }

    server
        .bind(("127.0.0.1", config::get().api.port))?
        .run()
        .await
}
