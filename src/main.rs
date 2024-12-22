#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

mod auth;
mod db;
mod models;
mod routes;

#[launch]
async fn rocket() -> _ {
    let pool = db::init_pool("blog.db").await.expect("Failed to initialize database pool");

    rocket::build()
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
        .mount("/assets", FileServer::from(relative!("assets")))
        .manage(pool)
        .attach(Template::fairing())
}
