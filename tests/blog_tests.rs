use rust_blog::{Post, BlogDb};
use tempfile::tempdir;
use std::fs;
use anyhow;

// This file will contain our integration tests
#[test]
fn test_placeholder() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test_db_creation() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let _db = BlogDb::new(&db_path)?;
    Ok(())
}

#[test]
fn test_create_post() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = BlogDb::new(&db_path)?;
    
    let post = Post::new("Test Title", "Test Content", "https://example.com/image.jpg");
    let id = db.create_post(&post)?;
    assert!(id > 0);
    
    Ok(())
}

#[test]
fn test_post_persistence() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = BlogDb::new(&db_path)?;
    
    let post = Post::new("Test Title", "Test Content", "https://example.com/image.jpg");
    let id = db.create_post(&post)?;
    
    let retrieved_post = db.get_post(id)?;
    assert_eq!(retrieved_post.title(), "Test Title");
    assert_eq!(retrieved_post.body(), "Test Content");
    assert_eq!(retrieved_post.image_url(), "https://example.com/image.jpg");
    
    Ok(())
}

#[test]
fn test_db_error_handling() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = BlogDb::new(&db_path)?;
    
    // Try to get a non-existent post
    let result = db.get_post(999);
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_get_post() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = BlogDb::new(&db_path)?;
    
    // Create multiple posts
    let post1 = Post::new("Title 1", "Content 1", "https://example.com/1.jpg");
    let post2 = Post::new("Title 2", "Content 2", "https://example.com/2.jpg");
    
    let id1 = db.create_post(&post1)?;
    let id2 = db.create_post(&post2)?;
    
    // Get and verify each post
    let retrieved1 = db.get_post(id1)?;
    assert_eq!(retrieved1.title(), "Title 1");
    assert_eq!(retrieved1.id(), Some(id1));
    
    let retrieved2 = db.get_post(id2)?;
    assert_eq!(retrieved2.title(), "Title 2");
    assert_eq!(retrieved2.id(), Some(id2));
    
    Ok(())
}

#[test]
fn test_list_posts_order() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = BlogDb::new(&db_path)?;
    
    // Create posts in a specific order
    let post1 = Post::new("Title 1", "Content 1", "https://example.com/1.jpg");
    let post2 = Post::new("Title 2", "Content 2", "https://example.com/2.jpg");
    let post3 = Post::new("Title 3", "Content 3", "https://example.com/3.jpg");
    
    db.create_post(&post1)?;
    db.create_post(&post2)?;
    db.create_post(&post3)?;
    
    // Get posts and verify order (newest first)
    let posts = db.list_posts()?;
    assert_eq!(posts.len(), 3);
    assert_eq!(posts[0].title(), "Title 3");
    assert_eq!(posts[1].title(), "Title 2");
    assert_eq!(posts[2].title(), "Title 1");
    
    Ok(())
}

#[test]
fn test_empty_database() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = BlogDb::new(&db_path)?;
    
    // Check empty list
    let posts = db.list_posts()?;
    assert!(posts.is_empty());
    
    // Try to get a post from empty DB
    let result = db.get_post(1);
    assert!(result.is_err());
    
    Ok(())
}
