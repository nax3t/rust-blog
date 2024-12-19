// This will contain our blog's core functionality
// We'll add code here as we implement features

use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;
use anyhow::Result;

#[derive(Debug)]
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

pub struct BlogDb {
    conn: Connection,
}

impl BlogDb {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Create the posts table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                image_url TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(BlogDb { conn })
    }
    
    pub fn create_post(&self, post: &Post) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO posts (title, body, image_url) VALUES (?1, ?2, ?3)"
        )?;
        
        let id = stmt.insert([&post.title, &post.body, &post.image_url])?;
        Ok(id)
    }
    
    pub fn get_post(&self, id: i64) -> Result<Post> {
        let mut stmt = self.conn.prepare(
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
}
