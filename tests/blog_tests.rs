use rust_blog::{BlogDb, Post};
use anyhow::Result;

// This file will contain our integration tests

fn setup_test_db() -> Result<BlogDb> {
    BlogDb::new_temporary()
}

#[test]
fn test_db_creation() -> Result<()> {
    let _ = setup_test_db()?;
    Ok(())
}

#[test]
fn test_empty_database() -> Result<()> {
    let db = setup_test_db()?;
    let posts = db.list_posts()?;
    assert!(posts.is_empty());
    Ok(())
}

#[test]
fn test_create_post() -> Result<()> {
    let db = setup_test_db()?;
    
    let post = Post::new(
        "Test Title",
        "Test Content",
        "https://example.com/test.jpg",
    );
    
    let id = db.create_post(&post)?;
    assert!(id > 0);
    Ok(())
}

#[test]
fn test_get_post() -> Result<()> {
    let db = setup_test_db()?;
    
    let post = Post::new(
        "Test Title",
        "Test Content",
        "https://example.com/test.jpg",
    );
    
    let id = db.create_post(&post)?;
    let retrieved = db.get_post(id)?;
    
    assert_eq!(retrieved.title(), post.title());
    assert_eq!(retrieved.body(), post.body());
    assert_eq!(retrieved.image_url(), post.image_url());
    
    Ok(())
}

#[test]
fn test_post_persistence() -> Result<()> {
    let db = setup_test_db()?;
    
    let post = Post::new(
        "Test Title",
        "Test Content",
        "https://example.com/test.jpg",
    );
    
    db.create_post(&post)?;
    
    let posts = db.list_posts()?;
    assert_eq!(posts.len(), 1);
    
    let retrieved = &posts[0];
    assert_eq!(retrieved.title(), post.title());
    assert_eq!(retrieved.body(), post.body());
    assert_eq!(retrieved.image_url(), post.image_url());
    
    Ok(())
}

#[test]
fn test_list_posts_order() -> Result<()> {
    let db = setup_test_db()?;
    
    let post1 = Post::new(
        "First Post",
        "First Content",
        "https://example.com/first.jpg",
    );
    let post2 = Post::new(
        "Second Post",
        "Second Content",
        "https://example.com/second.jpg",
    );
    
    db.create_post(&post1)?;
    db.create_post(&post2)?;
    
    let posts = db.list_posts()?;
    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].title(), "Second Post");
    assert_eq!(posts[1].title(), "First Post");
    
    Ok(())
}

#[test]
fn test_db_error_handling() -> Result<()> {
    let db = setup_test_db()?;
    let result = db.get_post(999);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_update_post() -> Result<()> {
    let db = setup_test_db()?;
    
    // Create initial post
    let post = Post::new(
        "Initial Title",
        "Initial content",
        "https://example.com/initial.jpg",
    );
    let post_id = db.create_post(&post)?;
    
    // Update the post
    let updated_post = Post::new(
        "Updated Title",
        "Updated content",
        "https://example.com/updated.jpg",
    );
    db.update_post(post_id, &updated_post)?;
    
    // Verify the update
    let retrieved_post = db.get_post(post_id)?;
    assert_eq!(retrieved_post.title(), "Updated Title");
    assert_eq!(retrieved_post.body(), "Updated content");
    assert_eq!(retrieved_post.image_url(), "https://example.com/updated.jpg");
    
    Ok(())
}

#[test]
fn test_update_nonexistent_post() -> Result<()> {
    let db = setup_test_db()?;
    
    let post = Post::new(
        "Title",
        "Content",
        "https://example.com/image.jpg",
    );
    
    // Try to update a post that doesn't exist
    let result = db.update_post(999, &post);
    assert!(result.is_err());
    
    Ok(())
}
