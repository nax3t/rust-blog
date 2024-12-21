# Quick Start Guide

This guide will help you get started with the Rust Blog application.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- SQLite3

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

## Running the Application

1. Start the server:
```bash
cargo run
```

2. Visit [http://localhost:8000](http://localhost:8000) in your browser

The server will automatically:
- Create the SQLite database if it doesn't exist
- Set up the required tables
- Start serving the blog application

## Basic Usage

### Creating a Post

1. Click "New Post" on the homepage
2. Fill in the form:
   - Title: Your post title
   - Content: Your post content (supports paragraphs)
   - Image URL: A valid http(s) URL to an image
3. Click "Create Post"

### Viewing Posts

- Visit `/posts` to see all posts
- Click a post title to view its details
- Posts are displayed in reverse chronological order

### Editing a Post

1. View the post you want to edit
2. Click "Edit Post"
3. Modify the form fields
4. Click "Update Post"

### Deleting a Post

1. View the post you want to delete
2. Click "Delete Post"
3. Confirm the deletion

## Database Management

The application uses SQLite with AUTOINCREMENT for post IDs:
- IDs are assigned sequentially
- Deleted post IDs are never reused
- Each post has a unique, permanent identifier

The database file is created at `blog.db` in the project root.

## Template Customization

Templates are located in the `templates` directory:
```
templates/
├── base.html.tera      # Base layout
├── error/             # Error pages
└── posts/            # Post-related templates
    ├── edit.html.tera
    ├── index.html.tera
    ├── new.html.tera
    └── show.html.tera
```

## Error Handling

The application provides helpful error messages for:
- Invalid form data
- Missing posts
- Server errors

## Next Steps

- Read the [API Documentation](../api/overview.md)
- Learn about the [Database Schema](../api/database.md)
- Check out the [Development Guide](../development/guide.md)
