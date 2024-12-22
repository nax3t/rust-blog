use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_dyn_templates::{Template, context};
use rocket::{get, post, put, delete, uri};
use uuid::Uuid;
use validator::Validate;

use crate::models::auth::AuthenticatedUser;
use crate::models::post::{CreatePost};
use crate::services::db::DbPool;
use crate::services::{post_service, comment_service};

#[get("/")]
pub async fn index(user: Option<AuthenticatedUser>, pool: &State<DbPool>) -> Template {
    let posts = post_service::get_posts(pool)
        .await
        .unwrap_or_else(|_| vec![]);

    Template::render("index", context! {
        user: user.map(|u| u.0),
        posts: posts,
        title: "Home",
    })
}

#[get("/posts/new")]
pub fn new_post(user: AuthenticatedUser) -> Template {
    Template::render("new_post", context! {
        user: user.0,
        title: "New Post",
    })
}

#[post("/posts", data = "<post>")]
pub async fn create_post(
    post: Form<CreatePost>,
    user: AuthenticatedUser,
    pool: &State<DbPool>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if let Err(e) = post.validate() {
        return Err(Flash::error(
            Redirect::to(uri!(new_post)),
            e.to_string(),
        ));
    }

    match post_service::create_post(pool, post.into_inner(), user.0.id).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to("/"),
            "Post created successfully!",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(new_post)),
            "Failed to create post",
        ))
    }
}

#[get("/posts/<id>")]
pub async fn get_post(
    id: &str,
    user: Option<AuthenticatedUser>,
    pool: &State<DbPool>,
) -> Result<Template, Flash<Redirect>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid post ID"))
    };

    let post = match post_service::get_post(pool, uuid).await {
        Ok(Some(post)) => post,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Post not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch post"))
    };

    let comments = comment_service::get_post_comments(pool, uuid)
        .await
        .unwrap_or_else(|_| vec![]);

    let title = post.title.clone();
    Ok(Template::render("post", context! {
        user: user.map(|u| u.0),
        post: post,
        comments: comments,
        title: &title,
    }))
}

#[get("/posts/<id>/edit")]
pub async fn edit_post_page(
    id: &str,
    user: AuthenticatedUser,
    pool: &State<DbPool>,
) -> Result<Template, Flash<Redirect>> {
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
        return Err(Flash::error(
            Redirect::to(format!("/posts/{}", id)),
            "You don't have permission to edit this post",
        ));
    }

    Ok(Template::render("edit_post", context! {
        user: user.0,
        post: post,
        title: "Edit Post",
    }))
}

#[put("/posts/<id>", data = "<post>")]
pub async fn update_post(
    id: &str,
    user: AuthenticatedUser,
    post: Form<CreatePost>,
    pool: &State<DbPool>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
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

    match post_service::update_post(pool, uuid, post.title.clone(), post.content.clone()).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(format!("/posts/{}", id)),
            "Post updated successfully!",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(format!("/posts/{}/edit", id)),
            "Failed to update post",
        ))
    }
}

#[delete("/posts/<id>")]
pub async fn delete_post(
    id: &str,
    user: AuthenticatedUser,
    pool: &State<DbPool>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
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
            Redirect::to(format!("/posts/{}", id)),
            "You don't have permission to delete this post",
        ));
    }

    match post_service::delete_post(pool, uuid).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to("/"),
            "Post deleted successfully!",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(format!("/posts/{}", id)),
            "Failed to delete post",
        ))
    }
}
