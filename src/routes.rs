use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::http::CookieJar;
use rocket::{get, post, put, State};
use rocket_dyn_templates::{Template, context};
use validator::Validate;
use crate::models::{CreateUser, LoginUser, CreatePost, CreateComment};
use crate::auth::AuthenticatedUser;
use crate::db::DbPool;
use crate::auth::{hash_password, verify_password};
use crate::db;

#[get("/")]
pub async fn index(_user: Option<AuthenticatedUser>, pool: &State<DbPool>) -> Template {
    let posts = match db::get_posts(pool).await {
        Ok(posts) => posts,
        Err(_) => vec![]
    };

    Template::render("index", context! {
        user: _user.map(|u| u.0),
        posts: posts
    })
}

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

    match db::create_user(pool, user.into_inner(), password_hash).await {
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
    let user = match db::get_user_by_username(pool, &credentials.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(Flash::error(Redirect::to("/login"), "Invalid username or password")),
        Err(_) => return Err(Flash::error(Redirect::to("/login"), "Database error"))
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

#[get("/posts/new")]
pub fn new_post(user: AuthenticatedUser) -> Template {
    Template::render("new_post", context! {
        user: user.0
    })
}

#[post("/posts", data = "<post>")]
pub async fn create_post(user: AuthenticatedUser, post: Form<CreatePost>, pool: &State<DbPool>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if let Err(e) = post.validate() {
        return Err(Flash::error(Redirect::to("/posts/new"), e.to_string()));
    }

    match db::create_post(pool, post.into_inner(), user.0.id).await {
        Ok(post) => Ok(Flash::success(Redirect::to(format!("/posts/{}", post.id)), "Post created successfully!")),
        Err(_) => Err(Flash::error(Redirect::to("/posts/new"), "Failed to create post"))
    }
}

#[get("/posts")]
pub async fn list_posts(pool: &State<DbPool>) -> Template {
    let posts = match db::get_posts(pool).await {
        Ok(posts) => posts,
        Err(_) => vec![]
    };

    Template::render("posts", context! {
        posts: posts
    })
}

#[get("/posts/<id>")]
pub async fn get_post(id: &str, _user: Option<AuthenticatedUser>, pool: &State<DbPool>) -> Option<Template> {
    let uuid = uuid::Uuid::parse_str(id).ok()?;
    let post = db::get_post(pool, uuid).await.ok()??;
    let comments = match db::get_post_comments(pool, uuid).await {
        Ok(comments) => comments,
        Err(_) => vec![]
    };

    Some(Template::render("post", context! {
        user: _user.map(|u| u.0),
        post: post,
        comments: comments
    }))
}

#[get("/posts/<id>/edit")]
pub async fn edit_post_page(id: &str, user: AuthenticatedUser, pool: &State<DbPool>) -> Option<Template> {
    let uuid = uuid::Uuid::parse_str(id).ok()?;
    let post = db::get_post(pool, uuid).await.ok()??;

    if post.author_id != user.0.id {
        return None;
    }

    Some(Template::render("edit_post", context! {
        user: user.0,
        post: post
    }))
}

#[put("/posts/<id>", data = "<post>")]
pub async fn update_post(id: &str, _user: AuthenticatedUser, post: Form<CreatePost>, pool: &State<DbPool>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uuid = match uuid::Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to(format!("/posts/{}/edit", id)), "Invalid post ID"))
    };

    if let Err(e) = post.validate() {
        return Err(Flash::error(Redirect::to(format!("/posts/{}/edit", id)), e.to_string()));
    }

    let post = post.into_inner();
    match db::update_post(pool, uuid, post.title, post.content).await {
        Ok(_) => Ok(Flash::success(Redirect::to(format!("/posts/{}", id)), "Post updated successfully!")),
        Err(_) => Err(Flash::error(Redirect::to(format!("/posts/{}/edit", id)), "Failed to update post"))
    }
}

#[post("/posts/<id>/delete")]
pub async fn delete_post(id: &str, user: AuthenticatedUser, pool: &State<DbPool>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uuid = match uuid::Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid post ID"))
    };

    // Check if the post exists and belongs to the user
    let post = match db::get_post(pool, uuid).await {
        Ok(Some(post)) => post,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Post not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch post"))
    };

    // Verify ownership
    if post.author_id != user.0.id {
        return Err(Flash::error(Redirect::to("/"), "You don't have permission to delete this post"));
    }

    match db::delete_post(pool, uuid).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Post deleted successfully!")),
        Err(_) => Err(Flash::error(Redirect::to(format!("/posts/{}", id)), "Failed to delete post"))
    }
}

#[post("/posts/<post_id>/comments", data = "<comment>")]
pub async fn create_comment(post_id: &str, user: AuthenticatedUser, comment: Form<CreateComment>, pool: &State<DbPool>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uuid = match uuid::Uuid::parse_str(post_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to(format!("/posts/{}", post_id)), "Invalid post ID"))
    };

    if let Err(e) = comment.validate() {
        return Err(Flash::error(Redirect::to(format!("/posts/{}", post_id)), e.to_string()));
    }

    let comment = comment.into_inner();
    match db::create_comment(pool, comment, uuid, user.0.id).await {
        Ok(_) => Ok(Flash::success(Redirect::to(format!("/posts/{}", post_id)), "Comment added successfully!")),
        Err(_) => Err(Flash::error(Redirect::to(format!("/posts/{}", post_id)), "Failed to add comment"))
    }
}

#[get("/posts/<post_id>/comments")]
pub async fn list_comments(post_id: &str, pool: &State<DbPool>) -> Template {
    let post_id = match uuid::Uuid::parse_str(post_id) {
        Ok(uuid) => uuid,
        Err(_) => return Template::render("error", context! { error: "Invalid post ID" })
    };

    let comments = match db::get_post_comments(pool, post_id).await {
        Ok(comments) => comments,
        Err(_) => return Template::render("error", context! { error: "Failed to retrieve comments" })
    };

    Template::render("comments", context! {
        comments: comments
    })
}