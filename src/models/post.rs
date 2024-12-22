use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use rocket::form::FromForm;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, FromForm)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}
