use serde::{Deserialize, Serialize};
use wither::{
    bson::{oid::ObjectId, DateTime},
    Model,
};

#[derive(Debug, Model, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    email: String,
}
