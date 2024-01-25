use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Scope};
use serde::Serialize;

use crate::{
    models::{user::Role, User},
    services::user::CreateUserData,
    utils::errors,
    AppState,
};

#[get("")]
async fn get_all_users(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let check_role_result = state
        .i
        .auth_service()
        .get_user_from_req_with_role(&req, Role::Admin)
        .await;

    if check_role_result.is_err() {
        return state.format_err(check_role_result.unwrap_err());
    }

    let result = state.i.post_service().list().await;

    match result {
        Ok(posts) => return HttpResponse::Ok().json(posts),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn get_user_by_id(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let check_role_result = state
        .i
        .auth_service()
        .get_user_from_req_with_role(&req, Role::Admin)
        .await;

    if check_role_result.is_err() {
        return state.format_err(check_role_result.unwrap_err());
    }

    let result = state.i.user_service().get_by_id(&path.into_inner().0).await;
    HttpResponse::Ok().json(result)
}

#[get("/me")]
async fn get_me(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    dbg!("HERE");
    let user = state.i.auth_service().get_user_from_req(&req).await;

    dbg!(&user);
    if user.is_err() {
        return state.format_err(user.unwrap_err());
    }

    return HttpResponse::Ok().json(user.unwrap());
}

#[derive(Serialize)]
struct CreateUserResponse {
    user: User,
    token: String,
}

#[post("")]
async fn create_user(
    state: web::Data<AppState>,
    user_data: web::Json<CreateUserData>,
) -> impl Responder {
    let result = state.i.user_service().create(user_data.into_inner()).await;

    if result.is_err() {
        return state.format_err(result.unwrap_err());
    }
    let user = result.unwrap();

    let token = state.i.auth_service().create_token(&user);

    if token.is_err() {
        return state.format_err(token.unwrap_err());
    }

    return HttpResponse::Ok().json(CreateUserResponse {
        token: token.unwrap(),
        user,
    });
}

// For very simplicity we omit password at all, just log in by providing email
#[post("/login")]
async fn login_user(
    state: web::Data<AppState>,
    user_data: web::Json<CreateUserData>,
) -> impl Responder {
    let user = state
        .i
        .user_service()
        .get_by_email(&user_data.into_inner().email)
        .await;

    let Some(user) = user else {
        return state.format_err(errors::build_generic_err());
    };

    let token = state.i.auth_service().create_token(&user);

    if token.is_err() {
        return state.format_err(token.unwrap_err());
    }

    return HttpResponse::Ok().json(CreateUserResponse {
        token: token.unwrap(),
        user,
    });
}

pub fn scope() -> Scope {
    let scope = web::scope("/users")
        .service(get_all_users)
        .service(create_user)
        .service(get_me)
        .service(get_user_by_id)
        .service(login_user);

    scope
}
