use rust_blog::{BlogDb, Post, App};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use hyper::body::to_bytes;
use tempfile::tempdir;
use tower::ServiceExt;
use anyhow::Result;

async fn setup_test_app() -> Result<(App, BlogDb)> {
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
    
    // Keep the TempDir alive
    std::thread::spawn(move || {
        let _dir = temp_dir;
        std::thread::park();
    });
    
    let app = App::new(db.clone());
    Ok((app, db))
}

#[tokio::test]
async fn test_index_page_exists() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();
    
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Get response body
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;
    
    // Check if test posts are displayed
    assert!(body.contains("Test Post 1"));
    assert!(body.contains("Content 1"));
    assert!(body.contains("Test Post 2"));
    assert!(body.contains("Content 2"));
    assert!(body.contains("https://example.com/1.jpg"));
    assert!(body.contains("https://example.com/2.jpg"));
    
    // Check if "New Post" link exists
    assert!(body.contains(r#"<a href='/posts/new'>New Post</a>"#));
    
    Ok(())
}

#[tokio::test]
async fn test_new_post_form_exists() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();
    
    let response = app
        .oneshot(Request::builder().uri("/posts/new").body(Body::empty())?)
        .await?;
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Get response body
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;
    
    // Check if form elements exist
    assert!(body.contains(r#"<form action="/posts" method="post">"#));
    assert!(body.contains(r#"<input type="text" id="title" name="title""#));
    assert!(body.contains(r#"<textarea id="body" name="body""#));
    assert!(body.contains(r#"<input type="url" id="image_url" name="image_url""#));
    assert!(body.contains(r#"<button type="submit">Create Post</button>"#));
    
    Ok(())
}

#[tokio::test]
async fn test_create_post_endpoint() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();
    
    let form_data = "title=New+Post&body=New+Content&image_url=https://example.com/new.jpg";
    let response = app
        .clone()
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
    let new_post = new_post.unwrap();
    assert_eq!(new_post.body(), "New Content");
    assert_eq!(new_post.image_url(), "https://example.com/new.jpg");
    
    Ok(())
}

#[tokio::test]
async fn test_post_validation() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();
    
    // Test missing title
    let form_data = "body=Content&image_url=https://example.com/img.jpg";
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(form_data))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    
    // Test missing body
    let form_data = "title=Title&image_url=https://example.com/img.jpg";
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(form_data))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    
    // Test invalid image URL
    let form_data = "title=Title&body=Content&image_url=not-a-url";
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(form_data))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    
    Ok(())
}

#[tokio::test]
async fn test_empty_index_page() -> Result<()> {
    // Create a new app with an empty database
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = BlogDb::new(&db_path)?;
    let app = App::new(db);
    let app = app.router();
    
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;
    
    // Check that we have the basic structure but no posts
    assert!(body.contains("<h1>Blog Posts</h1>"));
    assert!(body.contains("<ul></ul>"));
    assert!(body.contains(r#"<a href='/posts/new'>New Post</a>"#));
    
    Ok(())
}

#[tokio::test]
async fn test_malformed_post_data() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();
    
    // Test malformed content type
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("content-type", "text/plain")
                .body(Body::from("not-form-data"))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    
    // Test malformed form data
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from("not=valid&form=data"))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    
    Ok(())
}

#[tokio::test]
async fn test_html_escaping() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();
    
    // Create a post with HTML in the title and body
    let form_data = "title=<script>alert('xss')</script>&body=<p>html content</p>&image_url=https://example.com/img.jpg";
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(form_data))?,
        )
        .await?;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    // Verify the post was created and HTML is escaped
    let posts = db.list_posts()?;
    let post = posts.iter().find(|p| p.title().contains("script")).unwrap();
    assert!(post.title().contains("&lt;script&gt;"));
    assert!(post.body().contains("&lt;p&gt;"));
    
    // Check the index page
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;
    
    assert!(body.contains("&lt;script&gt;alert('xss')&lt;/script&gt;"));
    assert!(body.contains("&lt;p&gt;html content&lt;/p&gt;"));
    
    Ok(())
}
