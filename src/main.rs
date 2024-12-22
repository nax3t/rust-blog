#[macro_use]
extern crate rocket;

mod routes;
mod models;
mod services;
mod auth;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use crate::services::db::init_pool;

#[launch]
async fn rocket() -> _ {
    let pool = init_pool("blog.db").await.expect("Failed to create pool");

    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![
            routes::index,
            routes::register_page,
            routes::register,
            routes::login_page,
            routes::login,
            routes::logout,
            routes::new_post,
            routes::create_post,
            routes::get_post,
            routes::edit_post_page,
            routes::update_post,
            routes::delete_post,
            routes::create_comment,
            routes::edit_comment_page,
            routes::update_comment,
            routes::delete_comment,
        ])
        .manage(pool)
        .attach(Template::fairing())
}
