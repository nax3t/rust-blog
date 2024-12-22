mod common;
mod test_cleanup;

use uuid::Uuid;
use anyhow::Result;
use chrono::Utc;

#[tokio::test]
async fn test_post_crud() -> Result<()> {
    let (_db_name, pool) = common::setup_test_db().await?;
    let user_id = common::create_test_user(&pool, "testuser").await?;

    // Create post
    let post_id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO posts (id, title, content, author_id, author_username, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            post_id.to_string(),
            "Test Post",
            "This is a test post content.",
            user_id.to_string(),
            "testuser",
            now,
            now
        ],
    )?;

    // Read post
    let fetched_post = conn.query_row(
        "SELECT id, title, content, author_id, author_username
         FROM posts WHERE id = ?1",
        [post_id.to_string()],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        }
    )?;

    assert_eq!(fetched_post.0, post_id.to_string());
    assert_eq!(fetched_post.1, "Test Post");
    assert_eq!(fetched_post.2, "This is a test post content.");
    assert_eq!(fetched_post.3, user_id.to_string());
    assert_eq!(fetched_post.4, "testuser");

    // Update post
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE posts SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        rusqlite::params![
            "Updated Title",
            "Updated content.",
            now,
            post_id.to_string()
        ],
    )?;

    let updated_post = conn.query_row(
        "SELECT title, content FROM posts WHERE id = ?1",
        [post_id.to_string()],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
            ))
        }
    )?;

    assert_eq!(updated_post.0, "Updated Title");
    assert_eq!(updated_post.1, "Updated content.");

    // List posts
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM posts")?;
    let count: i32 = stmt.query_row([], |row| row.get(0))?;
    assert_eq!(count, 1);

    // Delete post
    conn.execute("DELETE FROM posts WHERE id = ?1", [post_id.to_string()])?;

    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM posts WHERE id = ?1",
        [post_id.to_string()],
        |row| row.get(0)
    )?;
    assert_eq!(count, 0);

    Ok(())
}
