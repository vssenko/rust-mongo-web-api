use std::rc::Rc;

use futures::{TryFutureExt, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
    Collection,
};
use serde::Deserialize;

use crate::models::{Database, DbError, Post};

#[derive(Debug)]
#[allow(unused)]
pub struct PostService {
    db: Rc<Database>,

    collection: Collection<Post>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostData {
    pub title: String,
    pub content: String,
}

impl PostService {
    pub async fn create(
        &self,
        post_data: CreatePostData,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        self.collection
            .insert_one(
                Post {
                    _id: ObjectId::new().to_hex(),
                    title: post_data.title,
                    content: post_data.content,
                },
                None,
            )
            .await
    }

    pub async fn list(&self) -> Result<Vec<Post>, DbError> {
        let result = self.collection.find(None, None).await;

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let result = result.unwrap();

        let posts: Vec<Post> = result.try_collect().unwrap_or_else(|_e| vec![]).await;

        Ok(posts)
    }

    pub async fn get_by_id(&self, id: &str) -> Option<Post> {
        let filter = doc! { "_id": id };
        let result = self.collection.find_one(filter, None).await;

        return result.unwrap_or_else(|_e| None);
    }

    pub fn new(db: Rc<Database>) -> Self {
        let collection = db.collection::<Post>("posts");
        PostService { db, collection }
    }
}
