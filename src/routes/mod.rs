pub mod auth;
pub mod posts;
pub mod profile;
pub mod comments;

use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
    routes![
        auth::login_page,
        auth::login,
        auth::register_page,
        auth::register,
        auth::logout,
        posts::index,
        posts::new_post,
        posts::create_post,
        posts::get_post,
        posts::edit_post_page,
        posts::update_post,
        posts::delete_post,
        profile::profile_page,
        profile::update_username,
        profile::update_password,
        comments::create_comment,
        comments::edit_comment_page,
        comments::update_comment,
        comments::delete_comment,
    ]
}
