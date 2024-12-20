use rocket::*;
use rocket::response::Redirect;
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
    // For now, just return an empty list while we test the setup
    Template::render("posts/index", context! {
        title: "Blog Posts",
        posts: Vec::<String>::new()
    })
}

/// Configure and build Rocket
pub fn rocket(db: BlogDb) -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, posts])
        .manage(RocketState::new(db))
        .attach(Template::fairing())
}
