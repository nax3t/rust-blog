mod common;

use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_post_crud() -> Result<()> {
    let (db_name, pool) = common::setup_test_db().await?;
    let user_id = common::create_test_user(&pool, "testuser").await?;

    // Create post
    let post_id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            post_id.to_string(),
            "Test Post",
            "This is a test post content.",
            user_id.to_string(),
            now,
            now
        ],
    )?;

    // Read post
    let fetched_post = conn.query_row(
        "SELECT p.id, p.title, p.content, p.author_id, u.username as author, p.created_at, p.updated_at
         FROM posts p
         JOIN users u ON p.author_id = u.id
         WHERE p.id = ?1",
        [post_id.to_string()],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
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
            "Updated Post Title",
            "Updated post content.",
            now,
            post_id.to_string()
        ],
    )?;

    let updated_post = conn.query_row(
        "SELECT title, content FROM posts WHERE id = ?1",
        [post_id.to_string()],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    )?;
    assert_eq!(updated_post.0, "Updated Post Title");
    assert_eq!(updated_post.1, "Updated post content.");

    // Delete post
    conn.execute(
        "DELETE FROM posts WHERE id = ?1",
        [post_id.to_string()],
    )?;

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM posts WHERE id = ?1",
        [post_id.to_string()],
        |row| row.get(0)
    )?;
    assert_eq!(count, 0);

    common::cleanup_test_db(&db_name);
    Ok(())
}

#[tokio::test]
async fn test_delete_user_cascades_to_posts() -> Result<()> {
    let (db_name, pool) = common::setup_test_db().await?;
    let user_id = common::create_test_user(&pool, "testuser").await?;

    // Create post
    let post_id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            post_id.to_string(),
            "Test Post",
            "This is a test post content.",
            user_id.to_string(),
            now,
            now
        ],
    )?;

    // Delete the user
    conn.execute(
        "DELETE FROM users WHERE id = ?1",
        [user_id.to_string()],
    )?;

    // Verify post was also deleted
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM posts WHERE id = ?1",
        [post_id.to_string()],
        |row| row.get(0)
    )?;
    assert_eq!(count, 0);

    common::cleanup_test_db(&db_name);
    Ok(())
}

#[tokio::test]
async fn test_delete_post_cascades_to_comments() -> Result<()> {
    let (db_name, pool) = common::setup_test_db().await?;
    let user_id = common::create_test_user(&pool, "testuser").await?;
    let commenter_id = common::create_test_user(&pool, "commenter").await?;

    // Create post
    let post_id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            post_id.to_string(),
            "Test Post",
            "This is a test post content.",
            user_id.to_string(),
            now,
            now
        ],
    )?;

    // Create comment
    let comment_id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO comments (id, content, post_id, author_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            comment_id.to_string(),
            "This is a test comment.",
            post_id.to_string(),
            commenter_id.to_string(),
            now,
            now
        ],
    )?;

    // Delete the post
    conn.execute(
        "DELETE FROM posts WHERE id = ?1",
        [post_id.to_string()],
    )?;

    // Verify comment was also deleted
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM comments WHERE id = ?1",
        [comment_id.to_string()],
        |row| row.get(0)
    )?;
    assert_eq!(count, 0);

    common::cleanup_test_db(&db_name);
    Ok(())
}
