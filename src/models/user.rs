use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use rocket::form::FromForm;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromForm, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Debug, FromForm)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, FromForm, Validate)]
pub struct UpdateUsername {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
}

#[derive(Debug, FromForm, Validate)]
pub struct UpdatePassword {
    pub current_password: String,
    #[validate(length(min = 6, message = "New password must be at least 6 characters long"))]
    pub new_password: String,
}
