use crate::models::post::{Post, CreatePost};
use crate::services::db::DbPool;
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;
use rusqlite::{params, OptionalExtension};

pub async fn create_post(pool: &DbPool, post: CreatePost, author_id: Uuid) -> Result<Post> {
    let conn = pool.get()?;
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4();

    // Get author username
    let author_username = conn.query_row(
        "SELECT username FROM users WHERE id = ?1",
        [author_id.to_string()],
        |row| row.get::<_, String>(0)
    )?;

    conn.execute(
        "INSERT INTO posts (id, title, content, author_id, author_username, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id.to_string(),
            post.title,
            post.content,
            author_id.to_string(),
            author_username,
            now,
            now
        ],
    )?;

    Ok(Post {
        id,
        title: post.title,
        content: post.content,
        author_id,
        author_username,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn get_posts(pool: &DbPool) -> Result<Vec<Post>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, title, content, author_id, author_username, created_at, updated_at
         FROM posts
         ORDER BY created_at DESC"
    )?;

    let posts = stmt.query_map([], |row| {
        Ok(Post {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            title: row.get(1)?,
            content: row.get(2)?,
            author_id: Uuid::parse_str(&row.get::<_, String>(3)?).unwrap(),
            author_username: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;

    let mut result = Vec::new();
    for post in posts {
        result.push(post?);
    }
    Ok(result)
}

pub async fn get_post(pool: &DbPool, id: Uuid) -> Result<Option<Post>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, title, content, author_id, author_username, created_at, updated_at
         FROM posts
         WHERE id = ?1"
    )?;

    let post = stmt.query_row([id.to_string()], |row| {
        Ok(Post {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            title: row.get(1)?,
            content: row.get(2)?,
            author_id: Uuid::parse_str(&row.get::<_, String>(3)?).unwrap(),
            author_username: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }).optional()?;

    Ok(post)
}

pub async fn update_post(pool: &DbPool, id: Uuid, title: String, content: String) -> Result<bool> {
    let conn = pool.get()?;
    let now = Utc::now().to_rfc3339();
    let rows = conn.execute(
        "UPDATE posts SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        params![title, content, now, id.to_string()],
    )?;
    Ok(rows > 0)
}

pub async fn delete_post(pool: &DbPool, id: Uuid) -> Result<bool> {
    let conn = pool.get()?;
    let rows = conn.execute("DELETE FROM posts WHERE id = ?1", [id.to_string()])?;
    Ok(rows > 0)
}
