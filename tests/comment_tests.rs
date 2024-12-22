mod common;

use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_comment_crud() -> Result<()> {
    let (db_name, pool) = common::setup_test_db().await?;
    let user_id = common::create_test_user(&pool, "testuser").await?;
    let commenter_id = common::create_test_user(&pool, "commenter").await?;

    // Create a post first
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

    // Read comment
    let fetched_comment = conn.query_row(
        "SELECT c.id, c.content, c.post_id, c.author_id, u.username as author, c.created_at, c.updated_at
         FROM comments c
         JOIN users u ON c.author_id = u.id
         WHERE c.id = ?1",
        [comment_id.to_string()],
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

    assert_eq!(fetched_comment.0, comment_id.to_string());
    assert_eq!(fetched_comment.1, "This is a test comment.");
    assert_eq!(fetched_comment.2, post_id.to_string());
    assert_eq!(fetched_comment.3, commenter_id.to_string());
    assert_eq!(fetched_comment.4, "commenter");

    // List comments for post
    let mut stmt = conn.prepare("SELECT id FROM comments WHERE post_id = ?1")?;
    let comments: Vec<String> = stmt.query_map([post_id.to_string()], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0], comment_id.to_string());

    // Update comment
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE comments SET content = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![
            "Updated comment content.",
            now,
            comment_id.to_string()
        ],
    )?;

    let updated_comment = conn.query_row(
        "SELECT content FROM comments WHERE id = ?1",
        [comment_id.to_string()],
        |row| row.get::<_, String>(0)
    )?;
    assert_eq!(updated_comment, "Updated comment content.");

    // Delete comment
    conn.execute(
        "DELETE FROM comments WHERE id = ?1",
        [comment_id.to_string()],
    )?;

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM comments WHERE id = ?1",
        [comment_id.to_string()],
        |row| row.get(0)
    )?;
    assert_eq!(count, 0);

    common::cleanup_test_db(&db_name);
    Ok(())
}

#[tokio::test]
async fn test_delete_user_cascades_to_comments() -> Result<()> {
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

    // Delete the user who made the comment
    conn.execute(
        "DELETE FROM users WHERE id = ?1",
        [commenter_id.to_string()],
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
