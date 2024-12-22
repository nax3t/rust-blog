#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::env;

mod models;
mod routes;
mod services;

use services::db::create_pool;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "blog.db".to_string());
    let pool = create_pool(&db_url);

    rocket::build()
        .mount("/", routes::routes())
        .mount("/static", FileServer::from(rocket::fs::relative!("static")))
        .manage(pool)
        .attach(Template::fairing())
}
