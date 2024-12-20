use rust_blog::{BlogDb, rocket};

#[rocket::main]
async fn main() {
    // Initialize the database
    let db = BlogDb::new("blog.db").expect("Failed to create database");
    
    // Build and launch the rocket
    if let Err(e) = rocket(db).launch().await {
        println!("Rocket failed to launch: {}", e);
        std::process::exit(1);
    }
}
