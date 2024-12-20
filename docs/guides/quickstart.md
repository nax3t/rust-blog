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

## Basic Usage

### Viewing Posts
- Visit the homepage at `http://localhost:3000` to see all posts
- Click on a post title to view its full content
- Posts are displayed in reverse chronological order (newest first)

### Creating Posts
1. Click "New Post" on the homepage
2. Fill out the form:
   - Title: Your post title
   - Content: Your post content
   - Image URL: A valid URL to an image
3. Click "Create Post"

### Editing Posts
1. View the post you want to edit
2. Click "Edit" below the post
3. Update the form fields:
   - Title: Update the post title
   - Content: Modify the post content
   - Image URL: Change the image URL
4. Click "Update Post"

## Security Features

The blog automatically:
- Escapes HTML in post content
- Sanitizes URLs to prevent dangerous protocols
- Validates all form inputs
- Handles form submissions securely

## Next Steps

- Read the [Configuration Guide](configuration.md) to customize your blog
- Check out the [Security Guide](security.md) for security best practices
- See the [Development Guide](../development/guide.md) to start contributing
