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
