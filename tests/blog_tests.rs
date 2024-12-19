use rust_blog::{Post, BlogDb};
use tempfile::tempdir;
use anyhow;

// This file will contain our integration tests
#[test]
fn test_placeholder() {
    // This is just a placeholder test
    assert!(true);
}

#[test]
fn test_create_post() {
    let post = Post::new(
        "My First Post",
        "This is the content of my first post.",
        "https://example.com/image.jpg",
    );

    assert_eq!(post.title(), "My First Post");
    assert_eq!(post.body(), "This is the content of my first post.");
    assert_eq!(post.image_url(), "https://example.com/image.jpg");
}

#[test]
fn test_post_persistence() -> anyhow::Result<()> {
    // Create a temporary directory for our test database
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Initialize the database
    let db = BlogDb::new(db_path)?;
    
    // Create and save a post
    let post = Post::new(
        "Test Post",
        "Test Content",
        "https://example.com/test.jpg",
    );
    let post_id = db.create_post(&post)?;
    
    // Retrieve the post and verify
    let retrieved_post = db.get_post(post_id)?;
    assert_eq!(retrieved_post.title(), post.title());
    assert_eq!(retrieved_post.body(), post.body());
    assert_eq!(retrieved_post.image_url(), post.image_url());
    
    Ok(())
}
