use rust_blog::{App, BlogDb};
use std::net::SocketAddr;
use axum::serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the database
    let db = BlogDb::new("blog.db")?;
    
    // Create the app
    let app = App::new(db);
    
    // Create the router
    let app = app.router();
    
    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app
    ).await?;
    
    Ok(())
}
