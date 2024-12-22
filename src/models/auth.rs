use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use uuid::Uuid;

use crate::models::User;
use crate::services::db::DbPool;
use crate::services::user_service;

pub struct AuthenticatedUser(pub User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_id = match request.cookies().get_private("user_id") {
            Some(cookie) => cookie.value().to_string(),
            None => return Outcome::Forward(Status::Unauthorized),
        };

        let user_id = match Uuid::parse_str(&user_id) {
            Ok(id) => id,
            Err(_) => return Outcome::Forward(Status::Unauthorized),
        };

        let pool = match request.rocket().state::<DbPool>() {
            Some(pool) => pool,
            None => return Outcome::Error((Status::InternalServerError, ())),
        };

        let user = match user_service::get_user_by_id(pool, user_id).await {
            Ok(Some(user)) => user,
            _ => return Outcome::Forward(Status::Unauthorized),
        };

        Outcome::Success(AuthenticatedUser(user))
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    Ok(verify(password, hash)?)
}
