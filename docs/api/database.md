# Database Schema

This document describes the database schema used in the Rust Blog project.

## Overview

The Rust Blog uses SQLite as its database engine. The database file is created automatically when the application first runs.

## Tables

### posts

The main table storing blog posts.

```sql
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    image_url TEXT NOT NULL
);
```

#### Columns

| Column    | Type    | Constraints      | Description                    |
|-----------|---------|------------------|--------------------------------|
| id        | INTEGER | PRIMARY KEY      | Auto-incrementing post ID      |
| title     | TEXT    | NOT NULL        | The title of the blog post     |
| body      | TEXT    | NOT NULL        | The content of the blog post   |
| image_url | TEXT    | NOT NULL        | URL to the post's image        |

## Indexes

Currently, the table uses only the default primary key index on the `id` column.

## Relationships

The current schema is simple and does not include any relationships. Future versions may add tables for:
- Comments (relating to posts)
- Categories
- Tags
- Users/Authors

## Data Access

### Connection Pool

The application uses `r2d2` for connection pooling:
```rust
pub struct BlogDb {
    pool: Pool<SqliteConnectionManager>,
}
```

### CRUD Operations

#### Create
```sql
INSERT INTO posts (title, body, image_url) VALUES (?1, ?2, ?3)
```

#### Read
```sql
-- Get single post
SELECT id, title, body, image_url FROM posts WHERE id = ?1

-- List all posts (newest first)
SELECT id, title, body, image_url FROM posts ORDER BY id DESC
```

## Data Validation

The following validations are performed before data is stored:
1. Title must not be empty
2. Body must not be empty
3. Image URL must be a valid URL
4. All text content is HTML-escaped

## Future Schema Changes

Planned database improvements:
1. Add timestamps (created_at, updated_at)
2. Add soft delete capability
3. Add user/author information
4. Add categories and tags
5. Add comment support

## Backup and Maintenance

### Backup
The SQLite database file (`blog.db`) can be backed up by:
1. Stopping the application
2. Copying the database file
3. Restarting the application

### Maintenance
Regular maintenance tasks:
1. Run VACUUM to reclaim space
2. Check for database corruption
3. Backup the database file
4. Monitor database size

## Migration Strategy

When schema changes are needed:
1. Create a new migration script
2. Test migration on a copy of production data
3. Backup the database
4. Apply migration
5. Verify data integrity

## Performance Considerations

1. Indexes will be added as query patterns emerge
2. Large text fields (body) might need optimization
3. Consider implementing pagination
4. Monitor query performance
5. Regular VACUUM operations
