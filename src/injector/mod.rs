use std::rc::Rc;

use wither::mongodb::Database;

use crate::models;
use crate::services;

pub async fn new() -> Injector {
    let db = models::db::connect().await;
    let db_rc = Rc::new(db.clone());
    let user_service = services::UserService::new(Rc::clone(&db_rc));
    let post_service = services::PostService::new(Rc::clone(&db_rc));

    Injector {
        single_db: Rc::clone(&db_rc),
        single_user_service: Rc::new(user_service),
        single_post_service: Rc::new(post_service),
    }
}

// fields on injector are used to store "singleton" services
#[derive(Debug)]
pub struct Injector {
    single_db: Rc<Database>,
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
}
