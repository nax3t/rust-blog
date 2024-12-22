use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_dyn_templates::{Template, context};
use rocket::{get, put, uri};
use validator::Validate;

use crate::models::auth::AuthenticatedUser;
use crate::services::db::DbPool;
use crate::services::user_service;
use crate::models::user::{UpdateUsername, UpdatePassword};
use rocket::http::CookieJar;

#[get("/profile")]
pub fn profile_page(user: AuthenticatedUser) -> Template {
    Template::render("profile", context! {
        user: user.0,
        title: "Profile Settings",
    })
}

#[put("/profile/username", data = "<username>")]
pub async fn update_username(
    username: Form<UpdateUsername>,
    mut user: AuthenticatedUser,
    pool: &State<DbPool>,
    cookies: &CookieJar<'_>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let username = username.into_inner();

    if let Err(e) = username.validate() {
        return Err(Flash::error(
            Redirect::to(uri!(profile_page)),
            e.to_string(),
        ));
    }

    match user_service::update_username(pool, user.0.id, username.username).await {
        Ok(updated_user) => {
            // Update the user in the cookie
            user.0 = updated_user;
            cookies.add_private(rocket::http::Cookie::new("user_id", user.0.id.to_string()));
            Ok(Flash::success(
                Redirect::to(uri!(profile_page)),
                "Username updated successfully",
            ))
        }
        Err(e) => {
            Err(Flash::error(
                Redirect::to(uri!(profile_page)),
                e.to_string(),
            ))
        }
    }
}

#[put("/profile/password", data = "<password>")]
pub async fn update_password(
    password: Form<UpdatePassword>,
    mut user: AuthenticatedUser,
    pool: &State<DbPool>,
    cookies: &CookieJar<'_>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let password = password.into_inner();

    if let Err(e) = password.validate() {
        return Err(Flash::error(
            Redirect::to(uri!(profile_page)),
            e.to_string(),
        ));
    }

    match user_service::update_password(
        pool,
        user.0.id,
        password.current_password,
        password.new_password,
        &user.0.password_hash,
    ).await {
        Ok(updated_user) => {
            // Update the user in the cookie
            user.0 = updated_user;
            cookies.add_private(rocket::http::Cookie::new("user_id", user.0.id.to_string()));
            Ok(Flash::success(
                Redirect::to(uri!(profile_page)),
                "Password updated successfully",
            ))
        }
        Err(e) => {
            Err(Flash::error(
                Redirect::to(uri!(profile_page)),
                e.to_string(),
            ))
        }
    }
}
