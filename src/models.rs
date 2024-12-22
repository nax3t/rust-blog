use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use rocket::form::FromForm;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, FromForm)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub author_username: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub author_username: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, FromForm)]
pub struct CreateComment {
    #[validate(length(min = 1))]
    pub content: String
}
