# Development Guide

This guide provides detailed information for developers who want to contribute to the Rust Blog project.

## Development Environment Setup

### Required Tools
- Rust (latest stable version)
- Cargo (comes with Rust)
- SQLite3
- Git
- A code editor with Rust support (VS Code with rust-analyzer recommended)

### Recommended Extensions
- rust-analyzer: Rust language support
- SQLite Viewer: For database inspection
- TOML: For Cargo.toml editing

## Project Structure

```
rust-blog/
├── src/
│   ├── lib.rs      # Core functionality
│   │   ├── Post struct and implementations
│   │   ├── BlogDb database operations
│   │   └── App web handlers and routing
│   └── main.rs     # Server setup
├── tests/
│   ├── blog_tests.rs   # Database tests
│   └── web_tests.rs    # Web endpoint tests
├── docs/           # Documentation
│   ├── api/        # API documentation
│   ├── guides/     # User guides
│   └── development/# Developer guides
├── Cargo.toml      # Dependencies
├── Cargo.lock      # Locked dependencies
└── blog.db         # SQLite database
```

## Key Components

### Post Struct
The core data structure representing a blog post:
```rust
pub struct Post {
    id: Option<i64>,
    title: String,
    body: String,
    image_url: String,
}
```

### Database Operations
The `BlogDb` struct provides database operations:
- Connection pooling with r2d2
- Create: Insert new posts
- Read: Get post by ID or list all
- Update: Modify existing posts

### Web Handlers
The `App` struct manages web routes:
- RESTful routing with proper HTTP methods
- Form handling with validation
- Method override for PUT requests
- Security middleware for XSS prevention

## Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Test Organization
- `blog_tests.rs`: Database operation tests
- `web_tests.rs`: HTTP endpoint tests
  - Form submission
  - Content validation
  - Error handling
  - Security features

## Security Considerations

### XSS Prevention
- HTML escaping on all output
- URL sanitization for dangerous protocols
- Input validation on all forms

### Method Override
- PUT requests via POST with _method parameter
- Secure handling of method override
- Form validation and sanitization

### Database Security
- Connection pooling for safe concurrent access
- Parameter binding to prevent SQL injection
- Input validation before database operations

## Contributing

### Pull Request Guidelines
1. Write tests for new features
2. Update documentation
3. Follow Rust formatting guidelines
4. Add entries to CHANGELOG.md

### Code Style
- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Document public interfaces
- Write meaningful commit messages

## Common Development Tasks

### Adding a New Feature
1. Create a new branch
2. Add tests first (TDD approach)
3. Implement the feature
4. Update documentation
5. Submit a pull request

### Modifying Database Schema
1. Update the Post struct
2. Modify database creation SQL
3. Update CRUD operations
4. Add migration scripts if needed
5. Update tests

### Adding New Routes
1. Add route handler in lib.rs
2. Add to router configuration
3. Create corresponding tests
4. Update API documentation

## Best Practices

### Code Style
- Follow Rust standard formatting (use `rustfmt`)
- Use meaningful variable names
- Add comments for complex logic
- Keep functions focused and small

### Error Handling
- Use `anyhow` for error propagation
- Provide meaningful error messages
- Handle all potential error cases
- Log errors appropriately

### Security
- Always escape user input
- Validate all form data
- Use prepared statements for SQL
- Keep dependencies updated

### Performance
- Use connection pooling
- Implement efficient database queries
- Consider pagination for large datasets
- Profile code when needed

## Debugging

### Common Issues
1. **Database Connection Errors**
   - Check SQLite file permissions
   - Verify connection string
   - Check for locked database

2. **Test Failures**
   - Use `RUST_BACKTRACE=1` for detailed traces
   - Check temporary file cleanup
   - Verify test database state

3. **Compilation Errors**
   - Update dependencies
   - Check for breaking changes
   - Verify trait implementations

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum)
- [SQLite Documentation](https://sqlite.org/docs.html)
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
