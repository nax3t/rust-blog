use rust_blog::{BlogDb, Post, App};
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use hyper::body::to_bytes;
use tower::ServiceExt;
use anyhow::Result;

async fn setup_test_app() -> Result<(App, BlogDb)> {
    let db = BlogDb::new_temporary()?;
    
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
    
    let app = App::new(db.clone());
    Ok((app, db))
}

#[tokio::test]
async fn test_index_page_exists() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();

    // Test root redirects to /posts
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(header::LOCATION).unwrap(),
        "/posts"
    );

    // Test /posts shows the posts list
    let response = app
        .oneshot(Request::builder().uri("/posts").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;

    assert!(body.contains("<h1>Blog Posts</h1>"));
    assert!(body.contains("<a href='/posts/new'>New Post</a>"));
    assert!(body.contains("Test Post 1"));
    assert!(body.contains("Test Post 2"));

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
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(form_data))?,
        )
        .await?;
    
    // Should redirect to home page
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get(header::LOCATION).unwrap().to_str().unwrap(), "/posts");
    
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
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
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
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
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
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(form_data))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    
    Ok(())
}

#[tokio::test]
async fn test_empty_index_page() -> Result<()> {
    // Create a new app with an empty database
    let db = BlogDb::new_temporary()?;
    let app = App::new(db);
    let app = app.router();
    
    // Test root redirects to /posts
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(header::LOCATION).unwrap(),
        "/posts"
    );

    // Test /posts shows empty list
    let response = app
        .oneshot(Request::builder().uri("/posts").body(Body::empty())?)
        .await?;
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;
    
    assert!(body.contains("<h1>Blog Posts</h1>"));
    assert!(body.contains("<ul></ul>"));
    assert!(body.contains("<a href='/posts/new'>New Post</a>"));
    
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
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
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
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(form_data))?,
        )
        .await?;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    
    // Verify the post was created with raw HTML
    let posts = db.list_posts()?;
    let post = posts.iter().find(|p| p.title().contains("<script>")).unwrap();
    assert_eq!(post.title(), "<script>alert('xss')</script>");
    assert_eq!(post.body(), "<p>html content</p>");
    
    // Check the index page has escaped HTML
    let response = app
        .oneshot(Request::builder().uri("/posts").body(Body::empty())?)
        .await?;
    
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;
    
    assert!(body.contains("&lt;script&gt;alert('xss')&lt;/script&gt;"));
    assert!(body.contains("&lt;p&gt;html content&lt;/p&gt;"));
    
    Ok(())
}

#[tokio::test]
async fn test_show_post() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();

    // Create a test post
    let post = Post::new(
        "Test Post Title",
        "Test Post Content with multiple paragraphs.\n\nSecond paragraph here.",
        "https://example.com/test.jpg",
    );
    let post_id = db.create_post(&post)?;

    // Test viewing the post
    let response = app
        .oneshot(Request::builder().uri(format!("/posts/{}", post_id)).body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;

    // Check post content is displayed
    assert!(body.contains("Test Post Title"));
    assert!(body.contains("Test Post Content with multiple paragraphs."));
    assert!(body.contains("Second paragraph here."));
    assert!(body.contains("https://example.com/test.jpg"));
    
    // Check navigation elements
    assert!(body.contains(r#"<a href="/posts">Back to Posts</a>"#));
    
    Ok(())
}

#[tokio::test]
async fn test_show_nonexistent_post() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();

    // Try to view a post that doesn't exist
    let response = app.clone()
        .oneshot(Request::builder().uri("/posts/999").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[tokio::test]
async fn test_show_post_invalid_id() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();

    // Try to view a post with invalid ID format
    let response = app
        .oneshot(Request::builder().uri("/posts/invalid").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    Ok(())
}

#[tokio::test]
async fn test_edit_post_form() -> Result<()> {
    let db = BlogDb::new_temporary()?;
    let app = App::new(db.clone()).router();

    // Create a post first
    let post = Post::new(
        "Original Title",
        "Original content",
        "https://example.com/original.jpg",
    );
    let post_id = db.create_post(&post)?;

    // Get the edit form
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/posts/{}/edit", post_id))
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;

    println!("Actual HTML: {}", body);

    // Form should have method="POST" and hidden _method field
    assert!(body.contains(r#"method="POST""#));
    assert!(body.contains(r#"<input type="hidden" name="_method" value="PUT">"#));

    // Form should have correct action URL
    assert!(body.contains(format!(r#"action="/posts/{}""#, post_id).as_str()));

    // Form should have pre-filled values
    assert!(body.contains(r#"value="Original Title""#));
    assert!(body.contains(r#">Original content</textarea>"#));
    assert!(body.contains(r#"value="https://example.com/original.jpg""#));

    Ok(())
}

#[tokio::test]
async fn test_update_post() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();

    // Create a test post
    let post = Post::new(
        "Original Title",
        "Original content",
        "https://example.com/original.jpg",
    );
    let post_id = db.create_post(&post)?;

    // Submit the edit form
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/posts/{}", post_id))
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(
                    "title=Updated+Title&body=Updated+content&image_url=https://example.com/updated.jpg"
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(header::LOCATION).unwrap().to_str().unwrap(),
        format!("/posts/{}", post_id)
    );

    // Verify the update
    let updated_post = db.get_post(post_id)?;
    assert_eq!(updated_post.title(), "Updated Title");
    assert_eq!(updated_post.body(), "Updated content");
    assert_eq!(updated_post.image_url(), "https://example.com/updated.jpg");

    Ok(())
}

#[tokio::test]
async fn test_edit_nonexistent_post() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();

    // Try to get edit form for non-existent post
    let response = app.clone()
        .oneshot(Request::builder().uri("/posts/999/edit").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Try to update non-existent post
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/posts/999")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(
                    "title=Title&body=Content&image_url=https://example.com/image.jpg"
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[tokio::test]
async fn test_update_post_validation() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();

    // Create a test post
    let post = Post::new(
        "Original Title",
        "Original content",
        "https://example.com/original.jpg",
    );
    let post_id = db.create_post(&post)?;

    // Test empty title
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/posts/{}", post_id))
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(
                    "title=&body=Content&image_url=https://example.com/image.jpg"
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Test invalid image URL
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/posts/{}", post_id))
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(
                    "title=Title&body=Content&image_url=not-a-url"
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[tokio::test]
async fn test_update_post_sanitizes_urls() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();

    // Create a post first
    let post = Post::new(
        "Original Title",
        "Original Body",
        "https://example.com/image.jpg",
    );
    let post_id = db.create_post(&post)?;

    // Try to update with dangerous content
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/posts/{}", post_id))
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(format!(
                    "title=<script>alert(1)</script>&body=<img src='javascript:alert(1)'>&image_url=javascript:alert(1)"
                )))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    // Verify the post was updated with sanitized content
    let updated = db.get_post(post_id)?;
    assert!(!updated.title().contains("javascript:"));
    assert!(!updated.body().contains("javascript:"));
    assert!(!updated.image_url().contains("javascript:"));
    assert!(updated.image_url().contains("#alert(1)"));

    Ok(())
}

#[tokio::test]
async fn test_posts_list() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();

    // Create some test posts in a specific order
    let posts = vec![
        Post::new(
            "First Post",
            "First post content",
            "https://example.com/first.jpg",
        ),
        Post::new(
            "Second Post",
            "Second post content",
            "https://example.com/second.jpg",
        ),
        Post::new(
            "Third Post",
            "Third post content",
            "https://example.com/third.jpg",
        ),
    ];

    // Insert posts and collect their IDs
    let mut post_ids = Vec::new();
    for post in posts {
        let id = db.create_post(&post)?;
        post_ids.push(id);
    }

    // Test /posts shows all posts
    let response = app
        .oneshot(Request::builder().uri("/posts").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;

    println!("Actual body content: {}", body);

    // Check page structure
    assert!(body.contains("<html>"));
    assert!(body.contains("<body>"));
    assert!(body.contains("<h1>Blog Posts</h1>"));
    assert!(body.contains("<ul>"));
    assert!(body.contains("</ul>"));
    assert!(body.contains("<a href='/posts/new'>New Post</a>"));

    // Check each post is displayed with correct content and links
    for (i, id) in post_ids.iter().enumerate() {
        let post_num = i + 1;
        assert!(body.contains(&format!(r#"<li id='post-{}'"#, id)));
        assert!(body.contains(&format!(r#"<a href='/posts/{}'"#, id)));
        assert!(body.contains(&format!("{} Post", match post_num {
            1 => "First",
            2 => "Second",
            3 => "Third",
            _ => unreachable!(),
        })));
        assert!(body.contains(&format!("{} post content", match post_num {
            1 => "First",
            2 => "Second",
            3 => "Third",
            _ => unreachable!(),
        })));
        assert!(body.contains(&format!("https://example.com/{}.jpg", match post_num {
            1 => "first",
            2 => "second",
            3 => "third",
            _ => unreachable!(),
        })));
    }

    Ok(())
}

#[tokio::test]
async fn test_posts_list_html_safety() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let app = app.router();

    // Test various XSS vectors
    let dangerous_posts = vec![
        // Test 1: Basic script injection and event handlers
        Post::new(
            "<script>alert(1)</script>",
            "<img src=x onerror=alert(1)>",
            "javascript:alert(1)",
        ),
        // Test 2: Mixed quotes and embedded JavaScript
        Post::new(
            "\"onclick='alert(2)'><button>Click",
            "<div style=\"background:url('javascript:alert(2)')\">",
            "data:text/html,<script>alert(2)</script>",
        ),
        // Test 3: Base64 and protocol variations
        Post::new(
            "<script src='data:,alert(3)'></script>",
            "<iframe src='vbscript:alert(3)'>",
            "data:image/svg+xml;base64,PHN2Zy9vbmxvYWQ9YWxlcnQoMyk+",
        ),
    ];

    // Create all test posts
    for post in dangerous_posts {
        db.create_post(&post)?;
    }

    // Test /posts escapes HTML
    let response = app
        .oneshot(Request::builder().uri("/posts").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;

    println!("Actual body content: {}", body);

    // 1. Verify dangerous content is properly escaped
    assert!(body.contains("&lt;script&gt;"));
    assert!(body.contains("&lt;img"));
    assert!(body.contains("&lt;iframe"));

    // 2. Verify no unescaped dangerous elements
    assert!(!body.contains("<script>"));
    assert!(!body.contains("<iframe>"));
    
    // 3. Verify dangerous URLs are sanitized
    assert!(body.contains("#alert"));          // URLs are sanitized
    assert!(!body.contains("javascript:"));    // No javascript: URLs
    assert!(!body.contains("data:"));         // No data: URLs
    assert!(!body.contains("vbscript:"));     // No vbscript: URLs

    // 4. Verify all dangerous URLs are replaced with #
    assert!(body.matches("src=\"#\"").count() == 3);  // All dangerous URLs should be replaced

    // 5. Verify safe URLs are preserved
    assert!(body.contains("https://example.com/1.jpg"));
    assert!(body.contains("https://example.com/2.jpg"));

    // 6. Verify structure is maintained
    assert!(body.contains("<ul>"));
    assert!(body.contains("</ul>"));
    assert!(body.contains("<img"));            // We have img tags
    assert!(body.contains("width='200'"));     // With proper attributes
    assert!(body.contains("alt=\"Post image\""));  // And alt text

    Ok(())
}

#[tokio::test]
async fn test_show_post_has_edit_button() -> Result<()> {
    let db = BlogDb::new_temporary()?;
    let app = App::new(db.clone()).router();

    // Create a post first with HTML content to test escaping
    let post = Post::new(
        "Test <script>alert('xss')</script>",
        "Test content",
        "https://example.com/image.jpg",
    );
    let post_id = db.create_post(&post)?;

    // Get the show post page
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/posts/{}", post_id))
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;

    // Check for edit button/link
    assert!(body.contains(format!("/posts/{}/edit", post_id).as_str()));
    assert!(body.contains("Edit Post"));

    // The edit link should be properly escaped
    assert!(!body.contains("<script>"));
    assert!(body.contains("&lt;script&gt;"));

    Ok(())
}

#[tokio::test]
async fn test_edit_form_has_method_override() -> Result<()> {
    let db = BlogDb::new_temporary()?;
    let app = App::new(db.clone()).router();

    // Create a post first
    let post = Post::new(
        "Test Title",
        "Test content",
        "https://example.com/image.jpg",
    );
    let post_id = db.create_post(&post)?;

    // Get the edit form
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/posts/{}/edit", post_id))
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;

    // Form should have a hidden _method field
    assert!(body.contains(r#"<input type="hidden" name="_method" value="PUT">"#));
    
    // Form action should be the correct endpoint
    assert!(body.contains(format!(r#"action="/posts/{}""#, post_id).as_str()));

    Ok(())
}

#[tokio::test]
async fn test_show_post_has_delete_button() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let _post = db.get_post(1)?;
    let app = app.router();

    let response = app
        .oneshot(Request::builder().uri("/posts/1").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = String::from_utf8(to_bytes(response.into_body()).await?.to_vec())?;
    
    // Check for delete form with proper method override
    assert!(body.contains(r#"<form action="/posts/1" method="post" style="display: inline">"#));
    assert!(body.contains(r#"<input type="hidden" name="_method" value="DELETE">"#));
    assert!(body.contains(r#"<button type="submit""#));
    assert!(body.contains("Delete Post"));

    Ok(())
}

#[tokio::test]
async fn test_delete_post() -> Result<()> {
    let (app, db) = setup_test_app().await?;
    let _post = db.get_post(1)?;
    let app = app.router();

    // Delete the post
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/posts/1")
                .body(Body::empty())?
        )
        .await?;

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(header::LOCATION).unwrap(),
        "/posts"
    );

    // Verify post is deleted
    assert!(db.get_post(1).is_err());

    Ok(())
}

#[tokio::test]
async fn test_delete_nonexistent_post() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/posts/999")
                .body(Body::empty())?
        )
        .await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[tokio::test]
async fn test_delete_post_invalid_id() -> Result<()> {
    let (app, _) = setup_test_app().await?;
    let app = app.router();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/posts/invalid")
                .body(Body::empty())?
        )
        .await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    Ok(())
}
