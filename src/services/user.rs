use std::rc::Rc;

use wither::mongodb::Database;

#[derive(Debug)]
pub struct UserService {
    db: Rc<Database>,
}

impl UserService {
    async fn create() {
        dbg!("User: create");
    }

    async fn list() {
        dbg!("User: list");
    }

    async fn get_by_id() {
        dbg!("User: get_by_id");
    }

    pub fn new(db: Rc<Database>) -> Self {
        UserService { db }
    }
}
