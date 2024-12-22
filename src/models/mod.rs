pub mod user;
pub mod post;
pub mod comment;

// Re-export the models for easier access
pub use user::User;
pub use post::Post;
pub use comment::Comment;
