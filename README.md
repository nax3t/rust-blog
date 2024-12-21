# Rust Blog

A secure and efficient blog engine written in Rust, featuring a SQLite backend and modern web interface.

## Features

- **Secure Content Handling**: Built-in XSS prevention with HTML escaping and URL sanitization
- **Database Integration**: SQLite backend with connection pooling and sequential ID generation
- **Modern Web Framework**: Built with Rocket for efficient request handling and templating
- **Complete CRUD Operations**: Create, Read, Update, and Delete blog posts
- **Comprehensive Testing**: Full test suite covering database operations and web endpoints
- **Form Validation**: Robust input validation and error handling
- **Image Support**: Support for blog posts with images via URLs
- **Template Engine**: Dynamic HTML generation with Tera templates
- **RESTful Routes**: Clean URL structure following REST conventions

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
The server will start at `http://localhost:8000`

### Routes

- `GET /`: Redirects to /posts
- `GET /posts`: View all blog posts
- `GET /posts/new`: New post form
- `POST /posts`: Create a new post
- `GET /posts/:id`: View a specific post
- `GET /posts/:id/edit`: Edit post form
- `PUT /posts/:id`: Update a post
- `DELETE /posts/:id`: Delete a post

### Database

The blog uses SQLite with AUTOINCREMENT for post IDs, ensuring that:
- IDs are assigned sequentially
- Deleted post IDs are not reused
- Each post has a unique, permanent identifier

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
