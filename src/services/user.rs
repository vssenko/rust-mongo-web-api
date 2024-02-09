use std::rc::Rc;

use actix_web::HttpRequest;
use futures::{TryFutureExt, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

use serde::Deserialize;

use crate::{
    models::{
        user::{Role, UserAuth},
        Database, DbError, User,
    },
    services::AuthService,
    utils::errors,
};

#[derive(Deserialize)]
pub struct CreateUserData {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
#[allow(unused)]
pub struct UserService {
    db: Rc<Database>,
    auth_service: Rc<AuthService>,
    user_collection: Collection<User>,
    user_auth_collection: Collection<UserAuth>,
}

impl UserService {
    pub async fn get_user_from_req(&self, req: &HttpRequest) -> Result<User, errors::Error> {
        let Some(auth_header) = req.headers().get("Authorization") else {
            return Err(errors::build_unauth_err());
        };

        let Ok(auth_header) = auth_header.to_str() else {
            return Err(errors::build_unauth_err());
        };

        if !auth_header.starts_with("Bearer ") {
            return Err(errors::build_unauth_err());
        }

        let token = auth_header.replace("Bearer ", "");
        let decoded_token = self.auth_service.decode_token(token.as_str());

        if decoded_token.is_err() {
            return Err(errors::build_unauth_err());
        }

        let decoded_token = decoded_token.unwrap();

        let user = self.get_by_id(&decoded_token.user_id).await;

        match user {
            Some(user) => return Ok(user),
            None => return Err(errors::build_unauth_err()),
        }
    }

    pub async fn get_user_from_req_with_role(
        &self,
        req: &HttpRequest,
        role: Role,
    ) -> Result<User, errors::Error> {
        let user = self.get_user_from_req(req).await;
        if user.is_err() {
            return Err(user.unwrap_err());
        }

        let user = user.unwrap();

        if !user.role.satisfy(role) {
            return Err(errors::build_unauth_err());
        }

        return Ok(user);
    }

    pub async fn create(&self, user_data: CreateUserData) -> Result<User, errors::Error> {
        let create_user_result = self
            .user_collection
            .insert_one(
                User {
                    _id: ObjectId::new().to_hex(),
                    email: user_data.email,
                    role: Role::User,
                },
                None,
            )
            .await;

        let Ok(result) = create_user_result else {
            return Err(errors::build_generic_err());
        };

        let create_auth_result = self
            .user_auth_collection
            .insert_one(
                UserAuth {
                    _id: ObjectId::new().to_hex(),
                    user_id: result.inserted_id.to_string(),
                    password_hash: self.auth_service.generate_hash(&user_data.password),
                },
                None,
            )
            .await;

        if create_auth_result.is_err() {
            return Err(errors::build_generic_err());
        }

        let user = self.get_by_id(result.inserted_id.as_str().unwrap()).await;

        if user.is_none() {
            return Err(errors::build_generic_err());
        }

        return Ok(user.unwrap());
    }

    pub async fn login(&self, user_data: CreateUserData) -> Result<User, errors::Error> {
        let user = self.get_by_email(&user_data.email).await;

        let Some(user) = user else {
            return Err(errors::build_unauth_err());
        };

        let user_auth = self
            .user_auth_collection
            .find_one(
                doc! {
                    "password_hash": self.auth_service.generate_hash(user_data.password.as_str()),
                    "user_id": user._id.clone()
                },
                None,
            )
            .await;

        let Ok(user_auth) = user_auth else {
            return Err(errors::build_unauth_err());
        };

        if user_auth.is_none() {
            return Err(errors::build_unauth_err());
        };

        return Ok(user);
    }

    pub async fn list(&self) -> Result<Vec<User>, DbError> {
        let find_result = self.user_collection.find(None, None).await;
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
            .user_collection
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
            .user_collection
            .find_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await;

        return find_result.unwrap_or_else(|_e| None);
    }

    pub fn new(db: Rc<Database>, auth_service: Rc<AuthService>) -> Self {
        let user_collection: Collection<User> = db.collection("users");
        let user_auth_collection: Collection<UserAuth> = db.collection("user_auths");
        UserService {
            db,
            auth_service,
            user_collection,
            user_auth_collection,
        }
    }
}
