use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::http::CookieJar;
use rocket::{get, post, State};
use rocket_dyn_templates::{Template, context};
use validator::Validate;
use crate::models::user::{CreateUser, LoginUser};
use crate::auth::{AuthenticatedUser, hash_password, verify_password};
use crate::services::db::DbPool;
use crate::services::user_service;

#[get("/register")]
pub fn register_page(_user: Option<AuthenticatedUser>) -> Template {
    Template::render("register", context! {
        user: _user.map(|u| u.0)
    })
}

#[post("/register", data = "<user>")]
pub async fn register(user: Form<CreateUser>, pool: &State<DbPool>, _cookies: &CookieJar<'_>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if let Err(e) = user.validate() {
        return Err(Flash::error(Redirect::to("/register"), e.to_string()));
    }

    let password_hash = match hash_password(&user.password) {
        Ok(hash) => hash,
        Err(_) => return Err(Flash::error(Redirect::to("/register"), "Failed to hash password"))
    };

    match user_service::create_user(pool, user.into_inner(), password_hash).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/login"), "Registration successful! Please login.")),
        Err(_) => Err(Flash::error(Redirect::to("/register"), "Username already taken"))
    }
}

#[get("/login")]
pub fn login_page(_user: Option<AuthenticatedUser>) -> Template {
    Template::render("login", context! {
        user: _user.map(|u| u.0)
    })
}

#[post("/login", data = "<credentials>")]
pub async fn login(credentials: Form<LoginUser>, pool: &State<DbPool>, _cookies: &CookieJar<'_>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let user = match user_service::get_user_by_username(pool, &credentials.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(Flash::error(Redirect::to("/login"), "Invalid username or password")),
        Err(_) => return Err(Flash::error(Redirect::to("/login"), "Failed to fetch user"))
    };

    match verify_password(&credentials.password, &user.password_hash) {
        Ok(true) => {
            let mut cookie = rocket::http::Cookie::new("user_id", user.id.to_string());
            cookie.set_http_only(true);
            _cookies.add_private(cookie);
            Ok(Flash::success(Redirect::to("/"), "Login successful!"))
        },
        _ => Err(Flash::error(Redirect::to("/login"), "Invalid username or password"))
    }
}

#[post("/logout")]
pub fn logout(_cookies: &CookieJar<'_>) -> Flash<Redirect> {
    let cookie = rocket::http::Cookie::new("user_id", "");
    _cookies.remove_private(cookie);
    Flash::success(Redirect::to("/"), "Logged out successfully!")
}
