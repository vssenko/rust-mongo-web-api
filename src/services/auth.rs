
use crate::models::User;
use crate::{config, utils::errors};

use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use std::time;

#[derive(Debug)]
pub struct AuthService {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
}

impl AuthService {
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

    pub fn generate_hash(&self, string: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(string.as_bytes());
        hasher.update(config::get().api.hash_salt.as_bytes());

        let hash = hasher.finalize();

        format!("{:x}", hash)
    }

    pub fn new() -> Self {
        AuthService {}
    }
}
