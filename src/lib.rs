use std::path::Path;
use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use axum::{
    routing::{get, post},
    Router,
    extract::{State, Form},
    response::{Html, Redirect, Response},
    http::{StatusCode, HeaderMap, header},
};
use serde::Deserialize;

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

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }
}

#[derive(Clone)]
pub struct BlogDb {
    pool: Pool<SqliteConnectionManager>,
}

impl BlogDb {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        
        // Create the posts table if it doesn't exist
        let conn = pool.get()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                image_url TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(BlogDb { pool })
    }
    
    pub fn create_post(&self, post: &Post) -> Result<i64> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "INSERT INTO posts (title, body, image_url) VALUES (?1, ?2, ?3)"
        )?;
        
        let id = stmt.insert([&post.title, &post.body, &post.image_url])?;
        Ok(id)
    }
    
    pub fn get_post(&self, id: i64) -> Result<Post> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, title, body, image_url FROM posts WHERE id = ?"
        )?;
        
        let post = stmt.query_row([id], |row| {
            Ok(Post {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                body: row.get(2)?,
                image_url: row.get(3)?,
            })
        })?;
        
        Ok(post)
    }

    pub fn list_posts(&self) -> Result<Vec<Post>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, title, body, image_url FROM posts ORDER BY id DESC"
        )?;
        
        let posts = stmt.query_map([], |row| {
            Ok(Post {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                body: row.get(2)?,
                image_url: row.get(3)?,
            })
        })?;
        
        let posts: Result<Vec<_>, _> = posts.collect();
        Ok(posts?)
    }
}

#[derive(Clone)]
pub struct App {
    db: BlogDb,
}

impl App {
    pub fn new(db: BlogDb) -> Self {
        App { db }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/", get(Self::index))
            .route("/posts/new", get(Self::new_post_form))
            .route("/posts", post(Self::create_post))
            .with_state(self)
    }

    async fn index(
        State(app): State<App>
    ) -> Result<Html<String>, StatusCode> {
        match app.db.list_posts() {
            Ok(posts) => {
                let html = format!(
                    "<html><body><h1>Blog Posts</h1><ul>{}</ul><a href='/posts/new'>New Post</a></body></html>",
                    posts.iter()
                        .map(|p| format!(
                            "<li><h2>{}</h2><p>{}</p><img src='{}' width='200'></li>",
                            p.title(), p.body(), p.image_url()
                        ))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                Ok(Html(html))
            },
            Err(e) => {
                eprintln!("Error listing posts: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            },
        }
    }

    async fn new_post_form() -> Result<Html<String>, StatusCode> {
        let html = r#"
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
                    <button type="submit">Create Post</button>
                </form>
            </body>
            </html>
        "#;
        Ok(Html(html.to_string()))
    }

    async fn create_post(
        State(app): State<App>,
        Form(form): Form<CreatePost>,
    ) -> Result<Response, StatusCode> {
        let post = Post::new(&form.title, &form.body, &form.image_url);
        match app.db.create_post(&post) {
            Ok(_) => {
                let mut headers = HeaderMap::new();
                headers.insert(header::LOCATION, "/".parse().unwrap());
                Ok(Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(header::LOCATION, "/")
                    .body(axum::body::Body::empty())
                    .unwrap())
            },
            Err(e) => {
                eprintln!("Error creating post: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            },
        }
    }
}

#[derive(Deserialize)]
pub struct CreatePost {
    title: String,
    body: String,
    image_url: String,
}
