use std::rc::Rc;

use futures::{TryFutureExt, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

use serde::Deserialize;

use crate::{
    models::{user::Role, Database, DbError, User},
    utils::errors,
};

#[derive(Deserialize)]
pub struct CreateUserData {
    pub email: String,
}

#[derive(Debug)]
#[allow(unused)]
pub struct UserService {
    db: Rc<Database>,
    collection: Collection<User>,
}

impl UserService {
    pub async fn create(&self, user_data: CreateUserData) -> Result<User, errors::Error> {
        let result = self
            .collection
            .insert_one(
                User {
                    _id: ObjectId::new().to_hex(),
                    email: user_data.email,
                    role: Role::User,
                },
                None,
            )
            .await;

        dbg!(&result);

        if result.is_err() {
            return Err(errors::build_generic_err());
        }

        let result = result.unwrap();

        dbg!(result.inserted_id.as_str());

        let user = self.get_by_id(result.inserted_id.as_str().unwrap()).await;

        dbg!(&user);

        if user.is_none() {
            return Err(errors::build_generic_err());
        }

        return Ok(user.unwrap());
    }

    pub async fn list(&self) -> Result<Vec<User>, DbError> {
        let find_result = self.collection.find(None, None).await;
        if find_result.is_err() {
            return Err(find_result.unwrap_err());
        };

        let users = find_result
            .unwrap()
            .try_collect()
            .unwrap_or_else(|_e| vec![])
            .await;
        return Ok(users);
    }

    pub async fn get_by_id(&self, id: &str) -> Option<User> {
        let find_result = self
            .collection
            .find_one(
                doc! {
                    "_id": id
                },
                None,
            )
            .await;

        return find_result.unwrap_or_else(|_e| None);
    }

    pub async fn get_by_email(&self, email: &str) -> Option<User> {
        let find_result = self
            .collection
            .find_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await;

        return find_result.unwrap_or_else(|_e| None);
    }

    pub fn new(db: Rc<Database>) -> Self {
        let collection = db.collection("users");
        UserService { db, collection }
    }
}
