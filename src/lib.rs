use std::path::Path;
use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
    extract::{State, Form, Path as AxumPath},
    response::{Html, IntoResponse, Redirect, Response},
    http::{self, StatusCode, header},
    middleware::{self, Next},
};
use serde::Deserialize;
use url::Url;
use hyper::Request;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use anyhow::{Result, anyhow};
use tempfile;
use html_escape::encode_text;

async fn method_override<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    if req.method() == http::Method::POST {
        if let Some(form) = req.extensions_mut().get::<Form<CreatePost>>() {
            let method = match form.0._method.as_deref() {
                Some("PUT") => Some(http::Method::PUT),
                Some("DELETE") => Some(http::Method::DELETE),
                _ => None,
            };
            if let Some(m) = method {
                *req.method_mut() = m;
            }
        }
    }
    next.run(req).await
}

#[derive(Debug, Clone)]
pub struct Post {
    id: Option<i64>,
    title: String,
    body: String,
    image_url: String,
}

impl Post {
    pub fn new(title: &str, body: &str, image_url: &str) -> Self {
        Post {
            id: None,
            title: title.to_string(),
            body: body.to_string(),
            image_url: image_url.to_string(),
        }
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    pub image_url: String,
    #[serde(default)]
    pub _method: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MethodOverrideForm {
    _method: Option<String>,
    title: Option<String>,
    body: Option<String>,
    image_url: Option<String>,
}

impl MethodOverrideForm {
    fn into_create_post(self) -> Option<CreatePost> {
        match (self.title, self.body, self.image_url) {
            (Some(title), Some(body), Some(image_url)) => Some(CreatePost {
                title,
                body,
                image_url,
                _method: self._method,
            }),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct App {
    db: Arc<BlogDb>,
}

impl App {
    pub fn new(db: BlogDb) -> Self {
        App {
            db: Arc::new(db),
        }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/", get(move |_: ()| async { 
                Redirect::to("/posts") 
            }))
            .route("/posts", get(move |state| Self::list_posts(state)))
            .route("/posts/new", get(move |state| Self::new_post(state)))
            .route("/posts", post(move |state, form| Self::create_post(state, form)))
            .route("/posts/:id", get(move |state, path| Self::show_post(state, path)))
            .route("/posts/:id/edit", get(move |state, path| Self::edit_post(state, path)))
            .route("/posts/:id", post(move |state, path, form| Self::handle_post_request(state, path, form)))
            .layer(middleware::from_fn(method_override))
            .with_state(self)
    }

    async fn list_posts(State(app): State<App>) -> Result<impl IntoResponse, StatusCode> {
        let posts = app.db.list_posts().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let mut html = String::from("<html><body>");
        html.push_str("<h1>Blog Posts</h1>");
        html.push_str("<ul>");
        
        for post in posts {
            // Sanitize dangerous URLs
            let image_url = match post.image_url() {
                url if url.starts_with("javascript:") || 
                     url.starts_with("data:") || 
                     url.starts_with("vbscript:") => "#",
                url => url,
            };

            // Sanitize the title and body to remove dangerous URLs
            let sanitize_content = |content: &str| {
                content.replace("javascript:", "#")
                      .replace("data:", "#")
                      .replace("vbscript:", "#")
            };

            let title = sanitize_content(post.title());
            let body = sanitize_content(post.body());

            html.push_str(&format!(
                r#"<li id='post-{}'><h2><a href='/posts/{}'>{}</a></h2><p>{}</p><img src="{}" width='200' alt="Post image"></li>"#,
                post.id().unwrap_or(0),
                post.id().unwrap_or(0),
                encode_text(&title),
                encode_text(&body),
                encode_text(image_url)
            ));
        }
        
        html.push_str("</ul>");
        html.push_str("<a href='/posts/new'>New Post</a>");
        html.push_str("</body></html>");
        
        Ok(Html(html))
    }

    async fn new_post(State(_app): State<App>) -> impl IntoResponse {
        Html(r#"
            <html>
                <body>
                    <h1>New Blog Post</h1>
                    <form action="/posts" method="post">
                        <div>
                            <label for="title">Title:</label><br>
                            <input type="text" id="title" name="title" required>
                        </div>
                        <div>
                            <label for="body">Content:</label><br>
                            <textarea id="body" name="body" required></textarea>
                        </div>
                        <div>
                            <label for="image_url">Image URL:</label><br>
                            <input type="url" id="image_url" name="image_url" required>
                        </div>
                        <div>
                            <button type="submit">Create Post</button>
                        </div>
                    </form>
                    <p><a href="/posts">Back to Posts</a></p>
                </body>
            </html>
        "#)
    }

    async fn create_post(State(app): State<App>, Form(form): Form<CreatePost>) -> Result<impl IntoResponse, StatusCode> {
        // Validate form data
        if form.title.trim().is_empty() || form.body.trim().is_empty() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        
        if !Url::parse(&form.image_url).is_ok() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        
        // Create post with raw HTML
        let post = Post::new(
            &form.title,
            &form.body,
            &form.image_url,
        );
        
        // Save the post
        match app.db.create_post(&post) {
            Ok(_) => Ok((StatusCode::SEE_OTHER, [(header::LOCATION, "/posts")]).into_response()),
            Err(e) => {
                eprintln!("Error creating post: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    async fn show_post(State(app): State<App>, AxumPath(id): AxumPath<String>) -> Result<impl IntoResponse, StatusCode> {
        // Parse and validate the ID
        let post_id = id.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?;
        
        match app.db.get_post(post_id) {
            Ok(post) => {
                let html = format!(
                    r#"
                    <html>
                        <body>
                            <article>
                                <h1>{}</h1>
                                <div class="content">
                                    {}
                                </div>
                                <div class="image">
                                    <img src="{}" alt="Post image">
                                </div>
                            </article>
                            <p>
                                <a href="/posts/{}/edit">Edit Post</a> |
                                <form action="/posts/{}" method="post" style="display: inline"><input type="hidden" name="_method" value="DELETE"><button type="submit" onclick="return confirm('Are you sure you want to delete this post?')">Delete Post</button></form> |
                                <a href="/posts">Back to Posts</a>
                            </p>
                        </body>
                    </html>
                    "#,
                    encode_text(post.title()),
                    post.body()
                        .split("\n\n")
                        .map(|p| format!("<p>{}</p>", encode_text(p)))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    encode_text(post.image_url()),
                    post.id().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
                    post.id().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
                );
                Ok(Html(html))
            },
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn edit_post(State(app): State<App>, AxumPath(id): AxumPath<String>) -> Result<impl IntoResponse, StatusCode> {
        // Parse and validate the ID
        let post_id = id.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?;
        
        match app.db.get_post(post_id) {
            Ok(post) => {
                let html = format!(
                    r#"<html><body>
                        <h1>Edit Post</h1>
                        <form action="/posts/{}" method="POST">
                            <input type="hidden" name="_method" value="PUT">
                            <div>
                                <label for="title">Title:</label><br>
                                <input type="text" id="title" name="title" value="{}" required>
                            </div>
                            <div>
                                <label for="body">Content:</label><br>
                                <textarea id="body" name="body" required>{}</textarea>
                            </div>
                            <div>
                                <label for="image_url">Image URL:</label><br>
                                <input type="url" id="image_url" name="image_url" value="{}" required>
                            </div>
                            <input type="submit" value="Update Post">
                        </form>
                        <a href="/posts/{}">Back</a>
                    </body></html>"#,
                    post_id,
                    encode_text(post.title()),
                    encode_text(post.body()),
                    encode_text(post.image_url()),
                    post_id
                );
                Ok(Html(html))
            },
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn update_post(
        State(app): State<App>,
        AxumPath(id): AxumPath<String>,
        Form(form): Form<CreatePost>,
    ) -> Result<Response, StatusCode> {
        // Parse and validate the ID
        let post_id = id.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?;
        
        // Validate form data
        if form.title.trim().is_empty() || form.body.trim().is_empty() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        
        if !Url::parse(&form.image_url).is_ok() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }

        // Sanitize dangerous URLs
        let sanitize_content = |content: &str| {
            content.replace("javascript:", "#")
                  .replace("data:", "#")
                  .replace("vbscript:", "#")
        };

        let title = sanitize_content(&form.title);
        let body = sanitize_content(&form.body);
        let image_url = sanitize_content(&form.image_url);
        
        // Create post with sanitized content
        let post = Post::new(
            &title,
            &body,
            &image_url
        );
        
        // Update the post
        match app.db.update_post(post_id, &post) {
            Ok(_) => Ok((StatusCode::SEE_OTHER, [(header::LOCATION, format!("/posts/{}", post_id))]).into_response()),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn delete_post(State(app): State<App>, AxumPath(id): AxumPath<String>) -> Result<Response, StatusCode> {
        // Parse and validate the ID
        let post_id = id.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?;
        
        // Delete the post
        match app.db.delete_post(post_id) {
            Ok(_) => Ok((StatusCode::SEE_OTHER, [(header::LOCATION, "/posts")]).into_response()),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn handle_post_request(
        State(app): State<App>,
        AxumPath(id): AxumPath<String>,
        Form(form): Form<MethodOverrideForm>,
    ) -> Result<Response, StatusCode> {
        match form._method.as_deref() {
            Some("DELETE") => Self::delete_post(State(app), AxumPath(id)).await,
            Some("PUT") => {
                let create_post = form.into_create_post().ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;
                Self::update_post(State(app), AxumPath(id), Form(create_post)).await
            }
            _ => Err(StatusCode::METHOD_NOT_ALLOWED),
        }
    }
}

#[derive(Clone)]
pub struct BlogDb {
    pool: Pool<SqliteConnectionManager>,
    _temp_dir: Option<Arc<tempfile::TempDir>>, // Keep temp dir alive
}

impl BlogDb {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        
        let db = BlogDb { 
            pool,
            _temp_dir: None,
        };
        db.setup_database()?;
        Ok(db)
    }

    pub fn new_temporary() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let manager = SqliteConnectionManager::file(&db_path);
        let pool = Pool::new(manager)?;
        
        let db = BlogDb { 
            pool,
            _temp_dir: Some(Arc::new(temp_dir)),
        };
        db.setup_database()?;
        Ok(db)
    }

    fn setup_database(&self) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                image_url TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_post(&self, post: &Post) -> Result<i64> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO posts (title, body, image_url) VALUES (?1, ?2, ?3)",
            [post.title(), post.body(), post.image_url()],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_post(&self, id: i64) -> Result<Post> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT title, body, image_url FROM posts WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        
        if let Some(row) = rows.next()? {
            let mut post = Post::new(
                &row.get::<_, String>(0)?,
                &row.get::<_, String>(1)?,
                &row.get::<_, String>(2)?,
            );
            post.set_id(id);
            Ok(post)
        } else {
            Err(anyhow!("Post not found"))
        }
    }

    pub fn list_posts(&self) -> Result<Vec<Post>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, title, body, image_url FROM posts ORDER BY id DESC")?;
        let rows = stmt.query_map([], |row| {
            let mut post = Post::new(
                &row.get::<_, String>(1)?,
                &row.get::<_, String>(2)?,
                &row.get::<_, String>(3)?,
            );
            post.set_id(row.get(0)?);
            Ok(post)
        })?;
        
        let mut posts = Vec::new();
        for post in rows {
            posts.push(post?);
        }
        Ok(posts)
    }

    pub fn update_post(&self, id: i64, post: &Post) -> Result<()> {
        let conn = self.pool.get()?;
        let rows_affected = conn.execute(
            "UPDATE posts SET title = ?1, body = ?2, image_url = ?3 WHERE id = ?4",
            [&post.title, &post.body, &post.image_url, &id.to_string()],
        )?;
        
        if rows_affected == 0 {
            Err(anyhow!("Post not found"))
        } else {
            Ok(())
        }
    }

    pub fn delete_post(&self, id: i64) -> Result<()> {
        let conn = self.pool.get()?;
        let rows_affected = conn.execute("DELETE FROM posts WHERE id = ?1", [id.to_string()])?;
        if rows_affected == 0 {
            return Err(anyhow!("Post not found"));
        }
        Ok(())
    }
}

pub mod rocket_app;

// Re-export rocket for convenience
pub use rocket_app::rocket;
