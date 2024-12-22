use rocket::routes;
use rocket_dyn_templates::Template;

mod auth;
mod db;
mod models;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
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
            routes::list_posts,
            routes::get_post,
            routes::edit_post_page,
            routes::update_post,
            routes::delete_post,
            routes::create_comment,
            routes::list_comments,
        ])
        .manage(pool)
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
