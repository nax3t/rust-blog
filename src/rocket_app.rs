use rocket::{self, get, post, uri, Request, State, catch, routes, catchers};
use rocket::response::{Redirect, status::{NotFound, Custom}};
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use crate::BlogDb;
use std::sync::Arc;
use ::serde::Serialize;

/// Form data for creating/updating posts
#[derive(FromForm, Serialize)]
#[serde(crate = "::serde")]
struct PostForm {
    title: String,
    body: String,
    image_url: String,
}

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
#[get("/posts/<id_str>")]
async fn show_post(id_str: &str, state: &State<RocketState>) -> Result<Template, NotFound<String>> {
    match id_str.parse::<i64>() {
        Ok(id) => match state.db.get_post(id) {
            Ok(post) => {
                let title = post.title.clone();
                Ok(Template::render("posts/show", context! {
                    title: title,
                    post: post
                }))
            },
            Err(_) => Err(NotFound("Post not found".to_string()))
        },
        Err(_) => Err(NotFound("Post not found".to_string()))
    }
}

/// Show new post form
#[get("/posts/new")]
async fn new_post() -> Template {
    println!("Loading new post form template");
    let template = Template::render("posts/new", context! {
        title: "Create New Post"
    });
    println!("Template loaded");
    template
}

/// Create a new post
#[post("/posts", data = "<form>")]
async fn create_post(form: Form<PostForm>, state: &State<RocketState>) -> Result<Redirect, Custom<Template>> {
    // Validate URL
    if !form.image_url.starts_with("http://") && !form.image_url.starts_with("https://") {
        return Err(Custom(
            Status::UnprocessableEntity,
            Template::render("posts/new", context! {
                title: "Create New Post",
                error: "Image URL must start with http:// or https://",
                form: &*form
            })
        ));
    }
    
    // Create post
    let post = crate::Post::new(&form.title, &form.body, &form.image_url);
    match state.db.create_post(&post) {
        Ok(id) => Ok(Redirect::to(uri!(show_post(id.to_string())))),
        Err(_) => Err(Custom(
            Status::UnprocessableEntity,
            Template::render("posts/new", context! {
                title: "Create New Post",
                error: "Failed to create post",
                form: &*form
            })
        ))
    }
}

/// Handle invalid form data
#[catch(422)]
fn form_validation(_req: &Request) -> Custom<Template> {
    Custom(Status::UnprocessableEntity, Template::render("posts/new", context! {
        title: "Create New Post",
        error: "Invalid form data"
    }))
}

/// Handle invalid IDs
#[catch(404)]
fn not_found(_req: &Request) -> NotFound<String> {
    NotFound("Post not found".to_string())
}

/// Handle invalid ID format
#[catch(500)]
fn internal_error(_req: &Request) -> NotFound<String> {
    NotFound("Post not found".to_string())
}

/// Forward invalid ID to 404
#[catch(default)]
fn default_catcher(_status: Status, _req: &Request) -> NotFound<String> {
    NotFound("Post not found".to_string())
}

/// Build the Rocket instance
pub fn rocket(db: BlogDb) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index, posts, new_post, show_post, create_post])
        .register("/", catchers![form_validation, not_found, internal_error, default_catcher])
        .manage(RocketState::new(db))
        .attach(Template::fairing())
}
