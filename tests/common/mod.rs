use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use uuid::Uuid;
use chrono;
use bcrypt;
use anyhow::Result;
use std::fs;
use std::path::Path;

type DbPool = Pool<SqliteConnectionManager>;

/// Creates a test database pool with a unique name for parallel testing
pub async fn setup_test_db() -> Result<(String, DbPool)> {
    let db_name = format!("test_{}.db", Uuid::new_v4());
    let manager = SqliteConnectionManager::file(&db_name);
    let pool = Pool::new(manager)?;
    
    // Create tables
    let conn = pool.get()?;
    conn.execute_batch("
        PRAGMA foreign_keys = ON;
        
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS posts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            author_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS comments (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            post_id TEXT NOT NULL,
            author_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
            FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
        );
    ")?;

    Ok((db_name.to_string(), pool))
}

/// Creates a test user and returns their ID
pub async fn create_test_user(pool: &DbPool, username: &str) -> Result<Uuid> {
    use bcrypt::{hash, DEFAULT_COST};
    let password_hash = hash("testpass123", DEFAULT_COST)?;
    let id = Uuid::new_v4();
    let now = chrono::Utc::now().to_rfc3339();

    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            id.to_string(),
            username,
            password_hash,
            now,
            now
        ],
    )?;
    
    Ok(id)
}

/// Cleanup test database
pub fn cleanup_test_db(db_name: &str) {
    if Path::new(db_name).exists() {
        fs::remove_file(db_name).ok();
    }
}
