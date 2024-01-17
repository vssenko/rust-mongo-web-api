use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
struct Status {
    status: String,
}

#[get("")]
async fn get_all_posts(state: web::Data<AppState>) -> impl Responder {
    state.i.post_service().list().await;
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

#[get("/{id}")]
async fn get_post_by_id(state: web::Data<AppState>) -> impl Responder {
    state.i.post_service().get_by_id().await;
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

#[post("")]
async fn create_post(state: web::Data<AppState>) -> impl Responder {
    state.i.post_service().create().await;
    HttpResponse::Ok()
}

pub fn scope() -> Scope {
    let scope = web::scope("/status")
        .service(get_all_posts)
        .service(create_post);

    scope
}
