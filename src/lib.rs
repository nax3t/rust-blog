use std::path::Path;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use anyhow::{Result, anyhow};
use serde::Serialize;

/// A blog post
#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub body: String,
    pub image_url: String,
}

impl Post {
    pub fn new(title: &str, body: &str, image_url: &str) -> Self {
        Self {
            id: None,
            title: title.to_string(),
            body: body.to_string(),
            image_url: image_url.to_string(),
        }
    }
}

/// Database connection pool
#[derive(Clone)]
pub struct BlogDb {
    pool: Pool<SqliteConnectionManager>,
}

impl BlogDb {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        let db = Self { pool };
        db.setup_database()?;
        Ok(db)
    }

    pub fn new_temporary() -> Result<Self> {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager)?;
        let db = Self { pool };
        db.setup_database()?;
        Ok(db)
    }

    fn setup_database(&self) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
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
            params![post.title, post.body, post.image_url],
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
            post.id = Some(id);
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
            post.id = Some(row.get(0)?);
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
            params![post.title, post.body, post.image_url, id],
        )?;
        
        if rows_affected == 0 {
            Err(anyhow!("Post not found"))
        } else {
            Ok(())
        }
    }

    pub fn delete_post(&self, id: i64) -> Result<()> {
        let conn = self.pool.get()?;
        let rows_affected = conn.execute("DELETE FROM posts WHERE id = ?1", [id])?;
        
        if rows_affected == 0 {
            Err(anyhow!("Post not found"))
        } else {
            Ok(())
        }
    }
}

// Rocket app module
pub mod rocket_app;

// Re-export rocket app
pub use crate::rocket_app::rocket;
