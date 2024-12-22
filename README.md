# Rust Blog API

A RESTful blog API built with Rust, featuring user authentication, posts, and comments.

## Features

- User Management
  - User registration and authentication
  - Password hashing with bcrypt
  - JWT-based authentication

- Blog Posts
  - Create, read, update, and delete posts
  - Posts are associated with authors
  - Automatic cascading deletes

- Comments
  - Comment on posts
  - Comments are associated with both posts and authors
  - Automatic cascading deletes

## Database Schema

### Users Table
```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

### Posts Table
```sql
CREATE TABLE posts (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    author_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);
```

### Comments Table
```sql
CREATE TABLE comments (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    post_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);
```

## Development

### Prerequisites
- Rust (latest stable version)
- SQLite

### Setup
1. Clone the repository
2. Run `cargo build`
3. Run `cargo test` to ensure everything is working

### Running Tests
```bash
cargo test
```

The test suite includes:
- Unit tests for all services
- Integration tests for database operations
- Cascade delete tests for referential integrity

### API Endpoints

#### Users
- POST `/api/users/register` - Register a new user
- POST `/api/users/login` - Login and receive JWT token

#### Posts
- GET `/api/posts` - List all posts
- GET `/api/posts/{id}` - Get a specific post
- POST `/api/posts` - Create a new post (requires authentication)
- PUT `/api/posts/{id}` - Update a post (requires authentication)
- DELETE `/api/posts/{id}` - Delete a post (requires authentication)

#### Comments
- GET `/api/posts/{post_id}/comments` - List comments for a post
- POST `/api/posts/{post_id}/comments` - Add a comment (requires authentication)
- PUT `/api/comments/{id}` - Update a comment (requires authentication)
- DELETE `/api/comments/{id}` - Delete a comment (requires authentication)

## Security Features
- Password hashing using bcrypt
- JWT-based authentication
- Input validation and sanitization
- Foreign key constraints for data integrity
- Proper error handling and logging
