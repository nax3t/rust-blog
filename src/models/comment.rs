use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use rocket::form::FromForm;

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
