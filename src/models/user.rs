use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn satisfy(&self, another: Role) -> bool {
        if another == Role::Admin {
            return *self == Role::Admin;
        }

        return true;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub role: Role,
    pub email: String,
}
