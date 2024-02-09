
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub _id: String,
    pub title: String,
    pub content: String,
    pub user_id: String,
}
