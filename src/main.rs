use std::net::SocketAddr;
use rust_blog::{App, BlogDb};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = BlogDb::new("blog.db")?;
    let app = App::new(db);
    let router = app.router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
