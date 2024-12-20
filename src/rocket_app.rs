use rocket::*;
use rocket::response::{Redirect, status::NotFound};
use rocket_dyn_templates::{Template, context};
use crate::BlogDb;
use std::sync::Arc;

/// State wrapper for Rocket
#[derive(Clone)]
pub struct RocketState {
    db: Arc<BlogDb>,
}

impl RocketState {
    pub fn new(db: BlogDb) -> Self {
        Self {
            db: Arc::new(db),
        }
    }
}

/// Index route - redirects to posts list
#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(posts))
}

/// List all posts
#[get("/posts")]
async fn posts(state: &State<RocketState>) -> Template {
    let posts = state.db.list_posts().unwrap_or_default();
    
    Template::render("posts/index", context! {
        title: "Blog Posts",
        posts: posts
    })
}

/// Show a single post
#[get("/posts/<id>")]
async fn show_post(id: i64, state: &State<RocketState>) -> Result<Template, NotFound<String>> {
    match state.db.get_post(id) {
        Ok(post) => {
            let title = post.title.clone();
            Ok(Template::render("posts/show", context! {
                title: title,
                post: post
            }))
        },
        Err(_) => Err(NotFound("Post not found".to_string()))
    }
}

/// Handle invalid IDs by returning 404
#[catch(422)]
fn unprocessable_entity(_req: &Request) -> NotFound<String> {
    NotFound("Post not found".to_string())
}

/// Build the Rocket instance
pub fn rocket(db: BlogDb) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index, posts, show_post])
        .register("/", catchers![unprocessable_entity])
        .manage(RocketState::new(db))
        .attach(Template::fairing())
}
