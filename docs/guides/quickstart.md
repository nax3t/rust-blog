# Quick Start Guide

This guide will help you get the Rust Blog up and running quickly.

## Prerequisites

Before you begin, ensure you have:
- Rust (latest stable version)
- Cargo (comes with Rust)
- SQLite3
- Git (for version control)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-blog.git
cd rust-blog
```

2. Build the project:
```bash
cargo build
```

## Running the Server

1. Start the server:
```bash
cargo run
```

2. The server will start at `http://localhost:3000`

## Creating Your First Post

1. Open your web browser and navigate to `http://localhost:3000`
2. Click the "New Post" link
3. Fill out the form with:
   - Title: Your post title
   - Content: Your post content
   - Image URL: A valid URL to an image
4. Click "Create Post"

## Basic Usage

### Viewing Posts
- Visit the homepage at `http://localhost:3000` to see all posts
- Posts are displayed in reverse chronological order (newest first)

### Creating Posts
- Click "New Post" from any page
- All fields (title, content, image URL) are required
- Image URLs must be valid URLs
- HTML in posts is automatically escaped for security

## Next Steps

- Read the [Configuration Guide](configuration.md) to customize your blog
- Check out the [Security Guide](security.md) for security best practices
- See the [Development Guide](../development/guide.md) to start contributing
