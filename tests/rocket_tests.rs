use rust_blog::{BlogDb, rocket};
use rocket::local::blocking::Client;
use rocket::http::Status;

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
