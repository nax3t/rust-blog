use rust_blog::{Post, BlogDb};
use tempfile::tempdir;
use std::fs;
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
fn test_db_creation() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Database shouldn't exist yet
    assert!(!db_path.exists());
    
    // Create database
    let db = BlogDb::new(&db_path)?;
    
    // Database should exist now
    assert!(db_path.exists());
    
    // Should be able to open existing database
    let db2 = BlogDb::new(&db_path)?;
    
    // Both instances should work
    let post = Post::new("Test", "Content", "http://example.com/img.jpg");
    let id = db.create_post(&post)?;
    let retrieved = db2.get_post(id)?;
    assert_eq!(retrieved.title(), post.title());
    
    Ok(())
}

#[test]
fn test_post_persistence() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = BlogDb::new(db_path)?;
    
    let post = Post::new(
        "Test Post",
        "Test Content",
        "https://example.com/test.jpg",
    );
    let post_id = db.create_post(&post)?;
    
    let retrieved_post = db.get_post(post_id)?;
    assert_eq!(retrieved_post.title(), post.title());
    assert_eq!(retrieved_post.body(), post.body());
    assert_eq!(retrieved_post.image_url(), post.image_url());
    
    let posts = db.list_posts()?;
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].title(), post.title());
    
    Ok(())
}

#[test]
fn test_db_error_handling() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Create and immediately delete the database file
    {
        let _db = BlogDb::new(&db_path)?;
        fs::remove_file(&db_path)?;
    }
    
    // Trying to get a post from a deleted database should fail
    let db = BlogDb::new(&db_path)?;
    assert!(db.get_post(1).is_err());
    
    Ok(())
}
