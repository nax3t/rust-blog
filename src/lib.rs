use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
    extract::{State, Form},
    response::{Html, Response, IntoResponse},
    http::{StatusCode, header},
    body::BoxBody,
};
use serde::Deserialize;
use url::Url;
use html_escape::encode_text;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

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
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    title: String,
    body: String,
    image_url: String,
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
            .route("/", get(Self::index))
            .route("/posts/new", get(Self::new_post))
            .route("/posts", post(Self::create_post))
            .with_state(self)
    }

    async fn index(
        State(app): State<App>
    ) -> Result<impl IntoResponse, StatusCode> {
        match app.db.list_posts() {
            Ok(posts) => {
                let html = format!(
                    "<html><body><h1>Blog Posts</h1><ul>{}</ul><a href='/posts/new'>New Post</a></body></html>",
                    posts.iter()
                        .map(|p| format!(
                            "<li id='post-{}'><h2>{}</h2><p>{}</p><img src='{}' width='200'></li>",
                            p.id().unwrap_or(0),
                            p.title(),  
                            p.body(),   
                            p.image_url() 
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

    async fn new_post() -> impl IntoResponse {
        Html(r#"
            <html>
                <body>
                    <h1>New Post</h1>
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
        "#)
    }

    async fn create_post(
        State(app): State<App>,
        Form(form): Form<CreatePost>,
    ) -> Result<impl IntoResponse, StatusCode> {
        // Validate form data
        if form.title.trim().is_empty() || form.body.trim().is_empty() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }

        // Validate URL
        if Url::parse(&form.image_url).is_err() {
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }

        // Create post with escaped HTML
        let post = Post::new(
            &encode_text(&form.title),
            &encode_text(&form.body),
            &encode_text(&form.image_url)
        );

        match app.db.create_post(&post) {
            Ok(_) => {
                Ok(Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(header::LOCATION, "/")
                    .body(BoxBody::default())
                    .unwrap())
            },
            Err(e) => {
                eprintln!("Error creating post: {:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            },
        }
    }
}

pub struct BlogDb {
    pool: Pool<SqliteConnectionManager>,
}

impl BlogDb {
    pub fn new(db_path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::new(manager)?;
        
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

    pub fn create_post(&self, post: &Post) -> anyhow::Result<i64> {
        let conn = self.pool.get()?;
        conn.execute(
            "INSERT INTO posts (title, body, image_url) VALUES (?1, ?2, ?3)",
            [&post.title, &post.body, &post.image_url],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_post(&self, id: i64) -> anyhow::Result<Post> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, title, body, image_url FROM posts WHERE id = ?1"
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

    pub fn list_posts(&self) -> anyhow::Result<Vec<Post>> {
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

impl Clone for BlogDb {
    fn clone(&self) -> Self {
        BlogDb {
            pool: self.pool.clone(),
        }
    }
}
