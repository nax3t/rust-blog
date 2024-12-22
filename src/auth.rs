use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::User;

#[derive(Debug)]
pub struct AuthenticatedUser(pub User);

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_id = request.cookies()
            .get_private("user_id")
            .map(|cookie| cookie.value().to_string());

        match user_id {
            None => Outcome::Error((Status::Unauthorized, AuthError::Missing)),
            Some(user_id) => {
                let pool = request.guard::<&State<DbPool>>().await.unwrap();
                let conn = pool.get().unwrap();
                
                let result = conn.query_row(
                    "SELECT id, username, password_hash, created_at, updated_at FROM users WHERE id = ?1",
                    [&user_id],
                    |row| {
                        Ok(User {
                            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                            username: row.get(1)?,
                            password_hash: row.get(2)?,
                            created_at: row.get(3)?,
                            updated_at: row.get(4)?,
                        })
                    },
                );

                match result {
                    Ok(user) => Outcome::Success(AuthenticatedUser(user)),
                    Err(_) => Outcome::Error((Status::Unauthorized, AuthError::Invalid)),
                }
            }
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password.as_bytes(), DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password.as_bytes(), hash)
}
