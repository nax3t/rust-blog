use rust_blog::{BlogDb, Post, App};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tempfile::tempdir;
use tower::ServiceExt;
use std::sync::Arc;

async fn setup_test_app() -> anyhow::Result<(App, BlogDb)> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = BlogDb::new(&db_path)?;
    
    // Create some test posts
    let post1 = Post::new(
        "Test Post 1",
        "Content 1",
        "https://example.com/1.jpg",
    );
    let post2 = Post::new(
        "Test Post 2",
        "Content 2",
        "https://example.com/2.jpg",
    );
    db.create_post(&post1)?;
    db.create_post(&post2)?;
    
    // Keep the TempDir alive by storing it in an Arc
    std::thread::spawn(move || {
        let _dir = temp_dir;
        std::thread::park();
    });
    
    let app = App::new(db.clone());
    Ok((app, db))
}

#[tokio::test]
async fn test_index_page_exists() -> anyhow::Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();
    
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn test_new_post_form_exists() -> anyhow::Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();
    
    let response = app
        .oneshot(Request::builder().uri("/posts/new").body(Body::empty())?)
        .await?;
    
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn test_create_post_endpoint() -> anyhow::Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();
    
    let form_data = "title=New+Post&body=New+Content&image_url=https://example.com/new.jpg";
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(form_data))?,
        )
        .await?;
    
    // Should redirect to home page
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get("location").unwrap(), "/");
    
    // Verify the post was created
    let posts = db.list_posts()?;
    let new_post = posts.iter().find(|p| p.title() == "New Post");
    assert!(new_post.is_some());
    
    Ok(())
}
