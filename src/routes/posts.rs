use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::{get, post, put, State};
use rocket_dyn_templates::{Template, context};
use uuid::Uuid;
use crate::models::CreatePost;
use crate::auth::AuthenticatedUser;
use crate::services::db::DbPool;
use crate::services::{post_service, comment_service};

#[get("/")]
pub async fn index(_user: Option<AuthenticatedUser>, pool: &State<DbPool>) -> Template {
    let posts = match post_service::get_posts(pool).await {
        Ok(posts) => posts,
        Err(_) => vec![]
    };

    Template::render("index", context! {
        user: _user.map(|u| u.0),
        posts: posts
    })
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

    match post_service::create_post(pool, post.into_inner(), user.0.id).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Post created successfully!")),
        Err(_) => Err(Flash::error(Redirect::to("/posts/new"), "Failed to create post"))
    }
}

#[get("/posts/<id>")]
pub async fn get_post(id: &str, _user: Option<AuthenticatedUser>, pool: &State<DbPool>) -> Result<Template, Flash<Redirect>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid post ID"))
    };

    let post = match post_service::get_post(pool, uuid).await {
        Ok(Some(post)) => post,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Post not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch post"))
    };

    let comments = match comment_service::get_post_comments(pool, uuid).await {
        Ok(comments) => comments,
        Err(_) => vec![]
    };

    Ok(Template::render("post", context! {
        user: _user.map(|u| u.0),
        post: post,
        comments: comments
    }))
}

#[get("/posts/<id>/edit")]
pub async fn edit_post_page(id: &str, user: AuthenticatedUser, pool: &State<DbPool>) -> Result<Template, Flash<Redirect>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid post ID"))
    };

    let post = match post_service::get_post(pool, uuid).await {
        Ok(Some(post)) => post,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Post not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch post"))
    };

    if post.author_id != user.0.id {
        return Err(Flash::error(Redirect::to("/"), "You don't have permission to edit this post"));
    }

    Ok(Template::render("edit_post", context! {
        user: user.0,
        post: post
    }))
}

#[put("/posts/<id>", data = "<post>")]
pub async fn update_post(id: &str, user: AuthenticatedUser, post: Form<CreatePost>, pool: &State<DbPool>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid post ID"))
    };

    // Verify post exists and user owns it
    let existing_post = match post_service::get_post(pool, uuid).await {
        Ok(Some(post)) => post,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Post not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch post"))
    };

    if existing_post.author_id != user.0.id {
        return Err(Flash::error(
            Redirect::to(format!("/posts/{}", id)),
            "You don't have permission to edit this post",
        ));
    }

    if let Err(e) = post.validate() {
        return Err(Flash::error(
            Redirect::to(format!("/posts/{}/edit", id)),
            e.to_string(),
        ));
    }

    let post = post.into_inner();
    match post_service::update_post(pool, uuid, post.title, post.content).await {
        Ok(_) => Ok(Flash::success(Redirect::to(format!("/posts/{}", id)), "Post updated successfully!")),
        Err(_) => Err(Flash::error(Redirect::to(format!("/posts/{}/edit", id)), "Failed to update post"))
    }
}

#[post("/posts/<id>/delete")]
pub async fn delete_post(id: &str, user: AuthenticatedUser, pool: &State<DbPool>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid post ID"))
    };

    // Verify post exists and user owns it
    let post = match post_service::get_post(pool, uuid).await {
        Ok(Some(post)) => post,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Post not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch post"))
    };

    if post.author_id != user.0.id {
        return Err(Flash::error(
            Redirect::to("/"),
            "You don't have permission to delete this post",
        ));
    }

    match post_service::delete_post(pool, uuid).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Post deleted successfully!")),
        Err(_) => Err(Flash::error(Redirect::to("/"), "Failed to delete post"))
    }
}
