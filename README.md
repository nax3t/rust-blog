# Rust Blog

A secure and efficient blog engine written in Rust, featuring a SQLite backend and modern web interface.

## Features

- **Secure Content Handling**: Built-in XSS prevention with HTML escaping
- **Database Integration**: SQLite backend with connection pooling
- **Modern Web Framework**: Built with Axum for efficient request handling
- **Comprehensive Testing**: Full test suite covering database operations and web endpoints
- **Form Validation**: Robust input validation and error handling
- **Image Support**: Support for blog posts with images via URLs

## Documentation

Comprehensive documentation is available in the [docs](docs) directory:

- [Quick Start Guide](docs/guides/quickstart.md)
- [API Documentation](docs/api/overview.md)
- [Development Guide](docs/development/guide.md)
- [Database Schema](docs/api/database.md)

For a complete overview of the documentation, see the [Documentation Index](docs/README.md).

## Development

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)
- SQLite3

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Running
```bash
cargo run
```
The server will start at `http://localhost:3000`

### API Endpoints

- `GET /`: View all blog posts
- `GET /posts/new`: Create a new post form
- `POST /posts`: Submit a new blog post

### Project Structure

```
rust-blog/
├── src/
│   ├── lib.rs      # Core functionality and web handlers
│   └── main.rs     # Server setup and configuration
├── tests/
│   ├── blog_tests.rs   # Database integration tests
│   └── web_tests.rs    # Web endpoint tests
├── docs/          # Comprehensive documentation
│   ├── guides/    # User guides and tutorials
│   ├── api/       # API and database documentation
│   └── development/ # Developer documentation
└── blog.db        # SQLite database (created on first run)
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

For detailed development guidelines, see our [Development Guide](docs/development/guide.md).

## Versioning
This project follows [SemVer](https://semver.org/) for versioning. For the versions available, see the tags on this repository.

## License
This project is licensed under the MIT License - see the LICENSE file for details
