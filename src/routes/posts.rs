use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    status: String,
}

#[get("")]
async fn get_all_posts() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

#[post("")]
async fn create_post() -> impl Responder {
    HttpResponse::Ok()
}

pub fn scope() -> Scope {
    let scope = web::scope("/status")
        .service(get_all_posts)
        .service(create_post);

    scope
}
