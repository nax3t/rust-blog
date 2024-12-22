use crate::models::{Comment, CreateComment, CreatePost, CreateUser, Post, User};
use anyhow::Result;
use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

pub type DbPool = Pool<SqliteConnectionManager>;

pub async fn init_pool(db_path: &str) -> Result<DbPool> {
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager)?;
    
    // Initialize the database with our tables
    let conn = pool.get()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            author_id TEXT NOT NULL,
            author_username TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS comments (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            post_id TEXT NOT NULL,
            author_id TEXT NOT NULL,
            author_username TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,
            FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Enable foreign key support
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    Ok(pool)
}

pub async fn create_user(pool: &DbPool, user: CreateUser, password_hash: String) -> Result<User> {
    let conn = pool.get()?;
    let now = Utc::now().to_rfc3339();
    let user = User {
        id: Uuid::new_v4(),
        username: user.username,
        password_hash,
        created_at: now.clone(),
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            user.id.to_string(),
            user.username,
            user.password_hash,
            user.created_at,
            user.updated_at
        ],
    )?;

    Ok(user)
}

pub async fn get_user_by_username(pool: &DbPool, username: &str) -> Result<Option<User>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, created_at, updated_at FROM users WHERE username = ?1",
    )?;
    
    let user = stmt.query_row(params![username], |row| {
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

pub async fn create_post(pool: &DbPool, post: CreatePost, author_id: Uuid) -> Result<Post> {
    let conn = pool.get()?;
    let now = Utc::now().to_rfc3339();
    
    // Get the author's username
    let mut stmt = conn.prepare("SELECT username FROM users WHERE id = ?1")?;
    let author_username = stmt.query_row(params![author_id.to_string()], |row| row.get::<_, String>(0))?;
    
    let post = Post {
        id: Uuid::new_v4(),
        title: post.title,
        content: post.content,
        author_id,
        author_username,
        created_at: now.clone(),
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO posts (id, title, content, author_id, author_username, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            post.id.to_string(),
            post.title,
            post.content,
            post.author_id.to_string(),
            post.author_username,
            post.created_at,
            post.updated_at
        ],
    )?;

    Ok(post)
}

pub async fn get_posts(pool: &DbPool) -> Result<Vec<Post>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT p.id, p.title, p.content, p.author_id, p.author_username, p.created_at, p.updated_at 
         FROM posts p 
         ORDER BY p.created_at DESC"
    )?;

    let posts = stmt
        .query_map([], |row| {
            Ok(Post {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                title: row.get(1)?,
                content: row.get(2)?,
                author_id: Uuid::parse_str(&row.get::<_, String>(3)?).unwrap(),
                author_username: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(posts)
}

pub async fn get_post(pool: &DbPool, id: Uuid) -> Result<Option<Post>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT p.id, p.title, p.content, p.author_id, p.author_username, p.created_at, p.updated_at 
         FROM posts p 
         WHERE p.id = ?1"
    )?;

    let post = stmt.query_row(params![id.to_string()], |row| {
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
    let rows = conn.execute("DELETE FROM posts WHERE id = ?1", params![id.to_string()])?;
    Ok(rows > 0)
}

pub async fn create_comment(pool: &DbPool, comment: CreateComment, post_id: Uuid, author_id: Uuid) -> Result<Comment> {
    let conn = pool.get()?;
    let now = Utc::now().to_rfc3339();

    // Get the author's username
    let mut stmt = conn.prepare("SELECT username FROM users WHERE id = ?1")?;
    let author_username = stmt.query_row(params![author_id.to_string()], |row| row.get::<_, String>(0))?;

    let comment = Comment {
        id: Uuid::new_v4(),
        content: comment.content,
        post_id,
        author_id,
        author_username,
        created_at: now.clone(),
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO comments (id, content, post_id, author_id, author_username, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            comment.id.to_string(),
            comment.content,
            comment.post_id.to_string(),
            comment.author_id.to_string(),
            comment.author_username,
            comment.created_at,
            comment.updated_at
        ],
    )?;

    Ok(comment)
}

pub async fn get_post_comments(pool: &DbPool, post_id: Uuid) -> Result<Vec<Comment>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT c.id, c.content, c.post_id, c.author_id, c.author_username, c.created_at, c.updated_at
         FROM comments c
         WHERE c.post_id = ?1
         ORDER BY c.created_at ASC"
    )?;

    let comments = stmt
        .query_map(params![post_id.to_string()], |row| {
            Ok(Comment {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                content: row.get(1)?,
                post_id: Uuid::parse_str(&row.get::<_, String>(2)?).unwrap(),
                author_id: Uuid::parse_str(&row.get::<_, String>(3)?).unwrap(),
                author_username: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(comments)
}

pub async fn get_comment(pool: &DbPool, id: Uuid) -> Result<Option<Comment>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT c.id, c.content, c.post_id, c.author_id, c.author_username, c.created_at, c.updated_at
         FROM comments c
         WHERE c.id = ?1"
    )?;

    let comment = stmt.query_row(params![id.to_string()], |row| {
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
    let rows = conn.execute("DELETE FROM comments WHERE id = ?1", params![id.to_string()])?;
    Ok(rows > 0)
}
