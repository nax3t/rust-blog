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
│   │   └── App web handlers
│   └── main.rs     # Server setup
├── tests/
│   ├── blog_tests.rs   # Database tests
│   └── web_tests.rs    # Web endpoint tests
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
- Connection pooling with r2d2
- CRUD operations in BlogDb struct
- SQLite backend with rusqlite

### Web Handlers
- Built with Axum framework
- Form handling and validation
- HTML templating
- XSS prevention

## Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test blog_tests
cargo test --test web_tests

# Run specific test
cargo test test_create_post
```

### Test Categories
1. **Database Tests** (`blog_tests.rs`)
   - Post creation and retrieval
   - Database error handling
   - Post ordering
   - Empty database handling

2. **Web Tests** (`web_tests.rs`)
   - Route testing
   - Form submission
   - Error handling
   - XSS prevention
   - Content type validation

### Writing Tests
- Use `tempfile` for temporary test databases
- Follow the existing test structure
- Include both success and failure cases
- Test edge cases and error conditions

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
