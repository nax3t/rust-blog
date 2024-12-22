use crate::models::{Comment, CreateComment};
use crate::services::db::DbPool;
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;
use rusqlite::params;

pub async fn create_comment(pool: &DbPool, comment: CreateComment, post_id: Uuid, author_id: Uuid) -> Result<Comment> {
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
        "INSERT INTO comments (id, content, post_id, author_id, author_username, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id.to_string(),
            comment.content,
            post_id.to_string(),
            author_id.to_string(),
            author_username,
            now,
            now
        ],
    )?;

    Ok(Comment {
        id,
        content: comment.content,
        post_id,
        author_id,
        author_username,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn get_post_comments(pool: &DbPool, post_id: Uuid) -> Result<Vec<Comment>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, content, post_id, author_id, author_username, created_at, updated_at 
         FROM comments 
         WHERE post_id = ? 
         ORDER BY created_at DESC"
    )?;

    let comments = stmt.query_map([post_id.to_string()], |row| {
        Ok(Comment {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            content: row.get(1)?,
            post_id: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap(),
            author_id: Uuid::parse_str(&row.get::<_, String>(3)?).unwrap(),
            author_username: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;

    let mut result = Vec::new();
    for comment in comments {
        result.push(comment?);
    }
    Ok(result)
}

pub async fn get_comment(pool: &DbPool, id: Uuid) -> Result<Option<Comment>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, content, post_id, author_id, author_username, created_at, updated_at
         FROM comments
         WHERE id = ?1"
    )?;

    let comment = stmt.query_row([id.to_string()], |row| {
        Ok(Comment {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            content: row.get(1)?,
            post_id: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap(),
            author_id: Uuid::parse_str(&row.get::<_, String>(3)?).unwrap(),
            author_username: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }).optional()?;

    Ok(comment)
}

pub async fn update_comment(pool: &DbPool, id: Uuid, content: String) -> Result<bool> {
    let conn = pool.get()?;
    let now = Utc::now().to_rfc3339();
    let rows = conn.execute(
        "UPDATE comments SET content = ?1, updated_at = ?2 WHERE id = ?3",
        params![content, now, id.to_string()],
    )?;
    Ok(rows > 0)
}

pub async fn delete_comment(pool: &DbPool, id: Uuid) -> Result<bool> {
    let conn = pool.get()?;
    let rows = conn.execute("DELETE FROM comments WHERE id = ?1", [id.to_string()])?;
    Ok(rows > 0)
}
