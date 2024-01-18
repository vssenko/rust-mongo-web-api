use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: bson::oid::ObjectId,
    pub title: String,
    pub content: String,
}
