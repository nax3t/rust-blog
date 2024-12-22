use crate::models::{User, CreateUser};
use crate::services::db::DbPool;
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;
use rusqlite::params;

pub async fn create_user(pool: &DbPool, user: CreateUser, password_hash: String) -> Result<User> {
    let conn = pool.get()?;
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4();

    conn.execute(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            id.to_string(),
            user.username,
            password_hash,
            now,
            now
        ],
    )?;

    Ok(User {
        id,
        username: user.username,
        password_hash,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn get_user_by_username(pool: &DbPool, username: &str) -> Result<Option<User>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, created_at, updated_at
         FROM users
         WHERE username = ?1"
    )?;

    let user = stmt.query_row([username], |row| {
        Ok(User {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            username: row.get(1)?,
            password_hash: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }).optional()?;

    Ok(user)
}
