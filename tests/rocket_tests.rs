use rust_blog::{BlogDb, rocket};
use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};

fn setup_client() -> (Client, BlogDb) {
    let db = BlogDb::new_temporary().expect("Failed to create test database");
    let client = Client::tracked(rocket(db.clone())).expect("Failed to create client");
    (client, db)
}

#[test]
fn test_empty_index_page() {
    let (client, _db) = setup_client();
    let response = client.get("/posts").dispatch();
    
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    
    // Basic checks for empty state
    assert!(body.contains("No posts yet"));
    assert!(body.contains("create one"));
    assert!(body.contains("Rust Blog")); // From base template
}

#[test]
fn test_posts_list() {
    let (client, db) = setup_client();
    
    // Create test posts
    let post1 = rust_blog::Post::new(
        "First Post",
        "Content of first post",
        "https://example.com/image1.jpg",
    );
    let post2 = rust_blog::Post::new(
        "Second Post",
        "Content of second post",
        "https://example.com/image2.jpg",
    );
    
    db.create_post(&post1).expect("Failed to create post 1");
    db.create_post(&post2).expect("Failed to create post 2");
    
    let response = client.get("/posts").dispatch();
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().unwrap();
    assert!(body.contains("First Post"));
    assert!(body.contains("Second Post"));
    assert!(body.contains("Content of first post"));
    assert!(body.contains("Content of second post"));
}

#[test]
fn test_show_post() {
    let (client, db) = setup_client();
    
    // Create a test post
    let post = rust_blog::Post::new(
        "Test Post",
        "This is the post content.\nIt has multiple lines.",
        "https://example.com/image.jpg",
    );
    let post_id = db.create_post(&post).expect("Failed to create post");
    
    let response = client.get(format!("/posts/{}", post_id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().unwrap();
    
    assert!(body.contains("Test Post")); // Title
    assert!(body.contains("This is the post content.")); // Content
    assert!(body.contains("https:&#x2F;&#x2F;example.com&#x2F;image.jpg")); // Image URL
    assert!(body.contains("Edit")); // Edit button
    assert!(body.contains("Delete")); // Delete button
}

#[test]
fn test_show_nonexistent_post() {
    let (client, _db) = setup_client();
    
    let response = client.get("/posts/999").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_show_post_invalid_id() {
    let (client, _db) = setup_client();
    
    let response = client.get("/posts/invalid").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_new_post_form() {
    let (client, _db) = setup_client();
    
    let response = client.get("/posts/new").dispatch();
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().unwrap();
    
    assert!(body.contains("Create New Post")); // Title
    assert!(body.contains(r#"<form action="/posts" method="POST">"#)); // Form
    assert!(body.contains(r#"<input type="text" id="title" name="title""#)); // Title field
    assert!(body.contains(r#"<textarea id="body" name="body""#)); // Body field
    assert!(body.contains(r#"<input type="url" id="image_url" name="image_url""#)); // Image URL field
}

#[test]
fn test_create_post() {
    let (client, _db) = setup_client();
    
    let form_data = [
        ("title", "New Test Post"),
        ("body", "This is a test post content"),
        ("image_url", "https://example.com/test.jpg"),
    ];
    
    let response = client.post("/posts")
        .header(ContentType::Form)
        .body(form_data.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&"))
        .dispatch();
    
    assert_eq!(response.status(), Status::SeeOther); // Redirect after successful creation
    
    // Get the redirect location
    let location = response.headers().get_one("Location").expect("No redirect location");
    assert!(location.starts_with("/posts/")); // Should redirect to the new post
    
    // Follow the redirect
    let response = client.get(location).dispatch();
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().unwrap();
    assert!(body.contains("New Test Post")); // Title
    assert!(body.contains("This is a test post content")); // Content
    assert!(body.contains("https:&#x2F;&#x2F;example.com&#x2F;test.jpg")); // Image URL
}

#[test]
fn test_create_post_validation() {
    let (client, _db) = setup_client();
    
    // Test with missing title
    let form_data = [
        ("body", "Test content"),
        ("image_url", "https://example.com/test.jpg"),
    ];
    
    let response = client.post("/posts")
        .header(ContentType::Form)
        .body(form_data.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&"))
        .dispatch();
    
    assert_eq!(response.status(), Status::UnprocessableEntity);
    
    // Test with invalid image URL
    let form_data = [
        ("title", "Test Post"),
        ("body", "Test content"),
        ("image_url", "not-a-url"),
    ];
    
    let response = client.post("/posts")
        .header(ContentType::Form)
        .body(form_data.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&"))
        .dispatch();
    
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn test_edit_post_form() {
    let (client, db) = setup_client();
    
    // Create a test post
    let post = rust_blog::Post::new("Test Post", "Test Content", "https://example.com/image.jpg");
    let id = db.create_post(&post).unwrap();
    
    let response = client.get(format!("/posts/{}/edit", id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().unwrap();
    assert!(body.contains("Edit Post")); // Title
    assert!(body.contains(&format!(r#"<form action="/posts/{}" method="POST">"#, id))); // Form
    assert!(body.contains(r#"value="Test Post""#)); // Title field
    assert!(body.contains(r#">Test Content</textarea>"#)); // Body field
    assert!(body.contains(r#"value="https:&#x2F;&#x2F;example.com&#x2F;image.jpg""#)); // Image URL field
}

#[test]
fn test_edit_post() {
    let (client, db) = setup_client();
    
    // Create a test post
    let post = rust_blog::Post::new("Test Post", "Test Content", "https://example.com/image.jpg");
    let id = db.create_post(&post).unwrap();
    
    let response = client.post(format!("/posts/{}", id))
        .header(ContentType::Form)
        .body("title=Updated+Title&body=Updated+Content&image_url=https://example.com/new.jpg")
        .dispatch();
    
    assert_eq!(response.status(), Status::SeeOther);
    
    // Verify the post was updated
    let updated = db.get_post(id).unwrap();
    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.body, "Updated Content");
    assert_eq!(updated.image_url, "https://example.com/new.jpg");
}

#[test]
fn test_edit_post_validation() {
    let (client, db) = setup_client();
    
    // Create a test post
    let post = rust_blog::Post::new("Test Post", "Test Content", "https://example.com/image.jpg");
    let id = db.create_post(&post).unwrap();
    
    // Test invalid image URL
    let response = client.post(format!("/posts/{}", id))
        .header(ContentType::Form)
        .body("title=Updated+Title&body=Updated+Content&image_url=invalid-url")
        .dispatch();
    
    assert_eq!(response.status(), Status::UnprocessableEntity);
    
    let body = response.into_string().unwrap();
    assert!(body.contains(r#"<div class="error">"#)); // Error container
    assert!(body.contains("Image URL must start with http:&#x2F;&#x2F; or https:&#x2F;&#x2F;")); // Error message
    
    // Verify the post was not updated
    let unchanged = db.get_post(id).unwrap();
    assert_eq!(unchanged.title, "Test Post");
}

#[test]
fn test_edit_nonexistent_post() {
    let (client, _db) = setup_client();
    
    let response = client.get("/posts/999/edit").dispatch();
    assert_eq!(response.status(), Status::NotFound);
    
    let response = client.post("/posts/999")
        .header(ContentType::Form)
        .body("title=Updated+Title&body=Updated+Content&image_url=https://example.com/new.jpg")
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_delete_post() {
    let (client, db) = setup_client();
    
    // Create a test post
    let post = rust_blog::Post::new(
        "Test Post",
        "Test Content",
        "https://example.com/image.jpg"
    );
    let id = db.create_post(&post).unwrap();
    
    // Verify post exists
    assert!(db.get_post(id).is_ok());
    
    // Send delete request
    let response = client.delete(format!("/posts/{}", id)).dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    
    // Verify redirect to posts index
    let location = response.headers().get_one("Location").expect("No redirect location");
    assert_eq!(location, "/posts");
    
    // Verify post is deleted
    assert!(db.get_post(id).is_err());
}

#[test]
fn test_delete_nonexistent_post() {
    let (client, _db) = setup_client();
    
    let response = client.delete("/posts/999").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_delete_post_invalid_id() {
    let (client, _db) = setup_client();
    
    let response = client.delete("/posts/invalid").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_show_post_has_delete_form() {
    let (client, db) = setup_client();
    
    // Create a test post
    let post = rust_blog::Post::new(
        "Test Post",
        "Test Content",
        "https://example.com/image.jpg"
    );
    let id = db.create_post(&post).unwrap();
    
    let response = client.get(format!("/posts/{}", id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().unwrap();
    
    // Check for delete form
    assert!(body.contains(&format!(r#"<form action="/posts/{}" method="POST" style="display: inline;">"#, id)));
    assert!(body.contains(r#"<input type="hidden" name="_method" value="DELETE">"#));
    assert!(body.contains(r#"<button type="submit" onclick="return confirm('Are you sure you want to delete this post?')" class="button danger">Delete</button>"#));
}

#[test]
fn test_post_id_autoincrement() {
    let (client, db) = setup_client();
    
    // Create first post
    let post1 = rust_blog::Post::new(
        "First Post",
        "Content",
        "https://example.com/1.jpg"
    );
    let id1 = db.create_post(&post1).unwrap();
    
    // Create second post
    let post2 = rust_blog::Post::new(
        "Second Post",
        "Content",
        "https://example.com/2.jpg"
    );
    let id2 = db.create_post(&post2).unwrap();
    
    // Delete first post
    db.delete_post(id1).unwrap();
    
    // Create third post
    let post3 = rust_blog::Post::new(
        "Third Post",
        "Content",
        "https://example.com/3.jpg"
    );
    let id3 = db.create_post(&post3).unwrap();
    
    // Verify IDs are sequential and don't reuse deleted ID
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3); // Should be 3, not 1
}
