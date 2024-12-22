use crate::models::User;
use crate::models::user::CreateUser;
use crate::services::db::DbPool;
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use uuid::Uuid;
use rusqlite::{params, OptionalExtension};

pub async fn create_user(pool: &DbPool, user: CreateUser, password_hash: String) -> Result<User> {
    let conn = pool.get()?;
    let now = Utc::now().naive_utc().to_string();
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
    let user = conn.query_row(
        "SELECT id, username, password_hash, created_at, updated_at FROM users WHERE username = ?",
        params![username],
        |row| {
            let id_str: String = row.get(0)?;
            let id = Uuid::parse_str(&id_str).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;
            
            Ok(User {
                id,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        },
    ).optional()?;
    Ok(user)
}

pub async fn get_user_by_id(pool: &DbPool, id: Uuid) -> Result<Option<User>> {
    let conn = pool.get()?;
    let user = conn.query_row(
        "SELECT id, username, password_hash, created_at, updated_at FROM users WHERE id = ?",
        params![id.to_string()],
        |row| {
            let id_str: String = row.get(0)?;
            let id = Uuid::parse_str(&id_str).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;
            
            Ok(User {
                id,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        },
    ).optional()?;
    Ok(user)
}

pub async fn update_username(
    pool: &DbPool,
    user_id: Uuid,
    username: String,
) -> Result<User> {
    let conn = pool.get()?;
    let now = Utc::now().naive_utc().to_string();
    let mut updated_user = get_user_by_id(pool, user_id).await?.ok_or_else(|| anyhow!("User not found"))?;

    // Check if username is already taken by another user
    if let Some(existing_user) = get_user_by_username(pool, &username).await? {
        if existing_user.id != user_id {
            return Err(anyhow!("Username is already taken"));
        }
    }

    conn.execute(
        "UPDATE users SET username = ?, updated_at = ? WHERE id = ?",
        params![
            username,
            now,
            user_id.to_string()
        ],
    )?;

    updated_user.username = username;
    updated_user.updated_at = now;
    Ok(updated_user)
}

pub async fn update_password(
    pool: &DbPool,
    user_id: Uuid,
    current_password: String,
    new_password: String,
    current_password_hash: &str,
) -> Result<User> {
    let conn = pool.get()?;
    let now = Utc::now().naive_utc().to_string();
    let mut updated_user = get_user_by_id(pool, user_id).await?.ok_or_else(|| anyhow!("User not found"))?;

    // Verify current password
    if !verify(&current_password, current_password_hash)? {
        return Err(anyhow!("Current password is incorrect"));
    }

    // Update password
    let new_password_hash = hash(&new_password, DEFAULT_COST)?;
    conn.execute(
        "UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?",
        params![
            new_password_hash,
            now,
            user_id.to_string()
        ],
    )?;

    updated_user.password_hash = new_password_hash;
    updated_user.updated_at = now;
    Ok(updated_user)
}
