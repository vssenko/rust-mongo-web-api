use actix_web::{get, post, web, HttpResponse, Responder, Scope};

use crate::{services::post::CreatePostData, AppState};

#[get("")]
async fn get_all_posts(state: web::Data<AppState>) -> impl Responder {
    let result = state.i.post_service().list().await;

    match result {
        Ok(posts) => return HttpResponse::Ok().json(posts),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn get_post_by_id(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let result = state.i.post_service().get_by_id(&path).await;
    HttpResponse::Ok().json(result)
}

#[post("")]
async fn create_post(
    state: web::Data<AppState>,
    post_data: web::Json<CreatePostData>,
) -> impl Responder {
    let result = state.i.post_service().create(post_data.into_inner()).await;
    match result {
        Ok(insert_info) => return HttpResponse::Ok().json(insert_info),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

pub fn scope() -> Scope {
    let scope = web::scope("/posts")
        .service(get_all_posts)
        .service(create_post)
        .service(get_post_by_id);

    scope
}
