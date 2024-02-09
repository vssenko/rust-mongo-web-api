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
#[allow(unused)]
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
