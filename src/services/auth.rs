use super::UserService;
use crate::models::User;
use crate::{config, models::user::Role, utils::errors};
use actix_web::cookie::time::Duration;
use actix_web::HttpRequest;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::time;

#[derive(Debug)]
pub struct AuthService {
    user_service: Rc<UserService>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: String,
    exp: usize,
}

impl AuthService {
    pub async fn get_user_from_req(&self, req: &HttpRequest) -> Result<User, errors::Error> {
        dbg!("get_user_from_req");
        let Some(auth_header) = req.headers().get("Authorization") else {
            return Err(errors::build_unauth_err());
        };

        dbg!(&auth_header);

        let Ok(auth_header) = auth_header.to_str() else {
            return Err(errors::build_unauth_err());
        };
        dbg!(&auth_header);

        if !auth_header.starts_with("Bearer ") {
            return Err(errors::build_unauth_err());
        }
        dbg!(&auth_header);

        let token = auth_header.replace("Bearer ", "");
        dbg!(&token);
        let decoded_token = self.decode_token(token.as_str());

        dbg!(&decoded_token);
        if decoded_token.is_err() {
            return Err(errors::build_unauth_err());
        }

        let decoded_token = decoded_token.unwrap();

        let user = self.user_service.get_by_id(&decoded_token.user_id).await;

        dbg!(&user);

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
        dbg!("get_user_from_req_with_role");
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

    pub fn create_token(&self, user: &User) -> Result<String, errors::Error> {
        let key = config::get().api.jwt_secret.as_bytes();

        let header = Header {
            alg: Algorithm::HS512,
            ..Default::default()
        };

        let time = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("How the fuck i am tired of impossible errors");

        let claim = Claims {
            user_id: user._id.clone(),
            exp: (time.as_secs() + 365 * 24 * 60 * 60) as usize,
        };

        let token = encode(&header, &claim, &EncodingKey::from_secret(key));

        match token {
            Ok(t) => return Ok(t),
            Err(_) => return Err(errors::build_generic_err()),
        }
    }

    pub fn decode_token(&self, token_str: &str) -> Result<Claims, Error> {
        let key = config::get().api.jwt_secret.as_bytes();
        let claims = match decode::<Claims>(
            token_str,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(c) => c.claims,
            Err(e) => return Err(e),
        };

        return Ok(claims);
    }

    pub fn new(user_service: Rc<UserService>) -> Self {
        AuthService { user_service }
    }
}
