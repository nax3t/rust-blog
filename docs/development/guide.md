# Development Guide

This guide provides detailed information for developers who want to contribute to the Rust Blog project.

## Project Structure

```
rust-blog/
├── src/
│   ├── lib.rs           # Core functionality and database
│   ├── rocket_app.rs    # Rocket web application
│   └── bin/
│       └── rocket_server.rs  # Server entry point
├── templates/           # Tera templates
│   ├── base.html.tera
│   ├── error/
│   │   ├── 404.html.tera
│   │   └── 500.html.tera
│   └── posts/
│       ├── edit.html.tera
│       ├── index.html.tera
│       ├── new.html.tera
│       └── show.html.tera
├── tests/              # Integration tests
│   └── rocket_tests.rs
├── docs/              # Documentation
├── Cargo.toml         # Dependencies
└── README.md         # Project overview
```

## Key Components

### Database Layer (`lib.rs`)

The database layer uses SQLite with connection pooling:

```rust
pub struct BlogDb {
    pool: Pool<SqliteConnectionManager>,
}
```

Key features:
- Connection pooling with r2d2
- AUTOINCREMENT for post IDs
- Transaction support
- Error handling with anyhow

### Web Application (`rocket_app.rs`)

Built with Rocket framework:

```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
            posts,
            new_post,
            create_post,
            show_post,
            edit_post,
            update_post,
            delete_post,
        ])
        .attach(Template::fairing())
        .attach(Shield::new())
}
```

Features:
- RESTful routing
- Form handling
- Error catchers
- Security headers
- Template rendering

### Templates

Uses Tera templating engine:
- Base template inheritance
- Error handling
- Form validation
- Cross-site scripting prevention

## Development Workflow

### Setting Up Development Environment

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-blog.git
cd rust-blog
```

2. Install dependencies:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

### Making Changes

1. Create a new branch:
```bash
git checkout -b feature/your-feature
```

2. Write tests first:
```rust
#[test]
fn test_your_feature() {
    // Setup
    let (client, db) = setup_client();
    
    // Test
    let response = client.get("/your-endpoint").dispatch();
    
    // Assert
    assert_eq!(response.status(), Status::Ok);
}
```

3. Implement your feature
4. Run tests:
```bash
cargo test
```

5. Format code:
```bash
cargo fmt
```

6. Run clippy:
```bash
cargo clippy
```

### Database Changes

When modifying the database schema:

1. Update the schema in `lib.rs`:
```rust
const SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        image_url TEXT NOT NULL
    )
";
```

2. Add migration tests
3. Update documentation

### Adding Routes

1. Define the route in `rocket_app.rs`:
```rust
#[get("/your/path")]
fn your_route() -> Template {
    // Implementation
}
```

2. Add to routes list:
```rust
.mount("/", routes![
    // ... existing routes
    your_route,
])
```

3. Create template if needed
4. Add tests

## Testing

### Unit Tests

Write unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // Test implementation
    }
}
```

### Integration Tests

Write integration tests in `tests/rocket_tests.rs`:

```rust
#[test]
fn test_endpoint() {
    let (client, _) = setup_client();
    let response = client.get("/endpoint").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
```

### Test Database

Use in-memory SQLite for tests:

```rust
let db = BlogDb::new_temporary()?;
```

## Error Handling

1. Use `anyhow::Result` for most functions
2. Create custom errors when needed
3. Map errors to appropriate HTTP responses
4. Provide user-friendly error messages

## Documentation

1. Document all public items
2. Keep README.md updated
3. Update CHANGELOG.md
4. Maintain API documentation

## Best Practices

1. **Code Style**
   - Follow Rust style guidelines
   - Use meaningful variable names
   - Keep functions focused and small

2. **Security**
   - Validate all inputs
   - Escape HTML output
   - Use HTTPS in production
   - Sanitize URLs

3. **Testing**
   - Write tests first
   - Test edge cases
   - Use meaningful test names
   - Keep tests independent

4. **Database**
   - Use transactions where appropriate
   - Handle connection errors
   - Clean up test data
   - Use AUTOINCREMENT for IDs

5. **Error Handling**
   - Provide helpful error messages
   - Log errors appropriately
   - Return appropriate status codes
   - Handle all error cases

## Deployment

1. Build for release:
```bash
cargo build --release
```

2. Configure database path
3. Set up reverse proxy
4. Configure HTTPS
5. Set up logging

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests
4. Implement changes
5. Submit pull request

## Resources

- [Rocket Documentation](https://rocket.rs/v0.5/guide/)
- [Tera Documentation](https://tera.netlify.app/)
- [SQLite Documentation](https://sqlite.org/docs.html)
- [Rust Book](https://doc.rust-lang.org/book/)
