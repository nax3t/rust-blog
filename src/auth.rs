use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use bcrypt::{hash, verify, DEFAULT_COST};
use anyhow::Result;
use uuid::Uuid;
use crate::services::db::DbPool;
use crate::models::User;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = request.rocket().state::<DbPool>().unwrap();
        let cookies = request.cookies();

        match cookies.get_private("user_id") {
            Some(cookie) => {
                let conn = match pool.get() {
                    Ok(conn) => conn,
                    Err(_) => return request::Outcome::Error((Status::InternalServerError, ()))
                };

                match Uuid::parse_str(cookie.value()) {
                    Ok(user_id) => {
                        let user = match conn.query_row(
                            "SELECT id, username, password_hash, created_at, updated_at FROM users WHERE id = ?1",
                            [user_id.to_string()],
                            |row| {
                                Ok(User {
                                    id: user_id,
                                    username: row.get(1)?,
                                    password_hash: row.get(2)?,
                                    created_at: row.get(3)?,
                                    updated_at: row.get(4)?,
                                })
                            },
                        ) {
                            Ok(user) => user,
                            Err(_) => return request::Outcome::Error((Status::InternalServerError, ()))
                        };

                        request::Outcome::Success(AuthenticatedUser(user))
                    }
                    Err(_) => request::Outcome::Error((Status::Unauthorized, ()))
                }
            }
            None => request::Outcome::Forward(Status::Unauthorized)
        }
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password.as_bytes(), DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    Ok(verify(password.as_bytes(), hash)?)
}
