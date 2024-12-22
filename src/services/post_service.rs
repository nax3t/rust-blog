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

    conn.execute(
        "INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            id.to_string(),
            post.title,
            post.content,
            author_id.to_string(),
            now,
            now
        ],
    )?;

    // Get the post with author info
    Ok(get_post(pool, id).await?.unwrap())
}

pub async fn get_posts(pool: &DbPool) -> Result<Vec<Post>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT p.id, p.title, p.content, p.author_id, u.username, p.created_at, p.updated_at
         FROM posts p
         LEFT JOIN users u ON u.id = p.author_id
         ORDER BY p.created_at DESC"
    )?;

    let posts = stmt.query_map([], |row| {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
            0,
            rusqlite::types::Type::Text,
            Box::new(e),
        ))?;

        let author_id_str: String = row.get(3)?;
        let author_id = Uuid::parse_str(&author_id_str).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
            3,
            rusqlite::types::Type::Text,
            Box::new(e),
        ))?;

        Ok(Post {
            id,
            title: row.get(1)?,
            content: row.get(2)?,
            author_id,
            author: row.get(4)?,
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
    let post = conn.query_row(
        "SELECT p.id, p.title, p.content, p.author_id, u.username, p.created_at, p.updated_at
         FROM posts p
         LEFT JOIN users u ON u.id = p.author_id
         WHERE p.id = ?1",
        [id.to_string()],
        |row| {
            let id_str: String = row.get(0)?;
            let id = Uuid::parse_str(&id_str).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

            let author_id_str: String = row.get(3)?;
            let author_id = Uuid::parse_str(&author_id_str).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                3,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

            Ok(Post {
                id,
                title: row.get(1)?,
                content: row.get(2)?,
                author_id,
                author: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    ).optional()?;

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
    let rows = conn.execute("DELETE FROM posts WHERE id = ?1", params![id.to_string()])?;
    Ok(rows > 0)
}
