use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_dyn_templates::{Template, context};
use rocket::{get, post, put, delete};
use uuid::Uuid;
use validator::Validate;

use crate::models::auth::AuthenticatedUser;
use crate::models::comment::CreateComment;
use crate::services::db::DbPool;
use crate::services::comment_service;

#[post("/posts/<post_id>/comments", data = "<comment>")]
pub async fn create_comment(post_id: &str, user: AuthenticatedUser, comment: Form<CreateComment>, pool: &State<DbPool>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uuid = match Uuid::parse_str(post_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid post ID"))
    };

    if let Err(e) = comment.validate() {
        return Err(Flash::error(
            Redirect::to(format!("/posts/{}", post_id)),
            e.to_string(),
        ));
    }

    match comment_service::create_comment(pool, comment.into_inner(), uuid, user.0.id).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(format!("/posts/{}", post_id)),
            "Comment added successfully!",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(format!("/posts/{}", post_id)),
            "Failed to add comment",
        ))
    }
}

#[get("/posts/<post_id>/comments/<comment_id>/edit")]
pub async fn edit_comment_page(
    post_id: &str,
    comment_id: &str,
    user: AuthenticatedUser,
    pool: &State<DbPool>,
) -> Result<Template, Flash<Redirect>> {
    let comment_id = match Uuid::parse_str(comment_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid comment ID"))
    };

    let comment = match comment_service::get_comment(pool, comment_id).await {
        Ok(Some(comment)) => comment,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Comment not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch comment"))
    };
    
    if comment.author_id != user.0.id {
        return Err(Flash::error(
            Redirect::to(format!("/posts/{}", post_id)),
            "You don't have permission to edit this comment"
        ));
    }
    
    Ok(Template::render(
        "edit_comment",
        context! {
            user: user.0,
            comment: comment,
        },
    ))
}

#[put("/posts/<post_id>/comments/<comment_id>", data = "<comment>")]
pub async fn update_comment(
    post_id: &str,
    comment_id: &str,
    user: AuthenticatedUser,
    comment: Form<CreateComment>,
    pool: &State<DbPool>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let comment_id = match Uuid::parse_str(comment_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid comment ID"))
    };

    // Verify comment exists and user owns it
    let existing_comment = match comment_service::get_comment(pool, comment_id).await {
        Ok(Some(comment)) => comment,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Comment not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch comment"))
    };

    if existing_comment.author_id != user.0.id {
        return Err(Flash::error(
            Redirect::to(format!("/posts/{}", post_id)),
            "You don't have permission to edit this comment",
        ));
    }

    if let Err(e) = comment.validate() {
        return Err(Flash::error(
            Redirect::to(format!("/posts/{}/comments/{}/edit", post_id, comment_id)),
            e.to_string(),
        ));
    }

    match comment_service::update_comment(pool, comment_id, comment.content.clone()).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(format!("/posts/{}", post_id)),
            "Comment updated successfully!",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(format!("/posts/{}/comments/{}/edit", post_id, comment_id)),
            "Failed to update comment",
        ))
    }
}

#[delete("/posts/<post_id>/comments/<comment_id>")]
pub async fn delete_comment(
    post_id: &str,
    comment_id: &str,
    user: AuthenticatedUser,
    pool: &State<DbPool>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let comment_id = match Uuid::parse_str(comment_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Invalid comment ID"))
    };

    // Verify comment exists and user owns it
    let comment = match comment_service::get_comment(pool, comment_id).await {
        Ok(Some(comment)) => comment,
        Ok(None) => return Err(Flash::error(Redirect::to("/"), "Comment not found")),
        Err(_) => return Err(Flash::error(Redirect::to("/"), "Failed to fetch comment"))
    };

    if comment.author_id != user.0.id {
        return Err(Flash::error(
            Redirect::to(format!("/posts/{}", post_id)),
            "You don't have permission to delete this comment",
        ));
    }

    match comment_service::delete_comment(pool, comment_id).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(format!("/posts/{}", post_id)),
            "Comment deleted successfully!",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(format!("/posts/{}", post_id)),
            "Failed to delete comment",
        ))
    }
}
