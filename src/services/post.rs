use std::rc::Rc;

use wither::mongodb::Database;

#[derive(Debug)]
pub struct PostService {
    db: Rc<Database>,
}

impl PostService {
    pub async fn create(&self) {
        dbg!("Post: create");
    }

    pub async fn list(&self) {
        dbg!("Post: list");
    }

    pub async fn get_by_id(&self) {
        dbg!("Post: get_by_id");
    }

    pub fn new(db: Rc<Database>) -> Self {
        PostService { db }
    }
}
