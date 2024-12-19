# API Overview

This document provides a comprehensive overview of the Rust Blog API.

## Base URL

All URLs referenced in the documentation have the base URL:
```
http://localhost:3000
```

## Endpoints

### List Posts
```
GET /
```

Returns the HTML index page containing all blog posts.

**Response**:
- `200 OK`: Successfully retrieved posts
- `500 Internal Server Error`: Server error occurred

### New Post Form
```
GET /posts/new
```

Returns the HTML form for creating a new blog post.

**Response**:
- `200 OK`: Successfully retrieved form

### Create Post
```
POST /posts
```

Creates a new blog post.

**Request Body** (form-urlencoded):
```
title=Post Title&body=Post Content&image_url=https://example.com/image.jpg
```

**Fields**:
- `title` (required): The post title
- `body` (required): The post content
- `image_url` (required): Valid URL to an image

**Response**:
- `303 See Other`: Successfully created post (redirects to index)
- `422 Unprocessable Entity`: Invalid form data
- `415 Unsupported Media Type`: Invalid content type
- `500 Internal Server Error`: Server error occurred

## Error Handling

The API uses standard HTTP status codes:

- `200-299`: Success
- `400-499`: Client errors
- `500-599`: Server errors

Common error responses:
- `422`: Missing or invalid form fields
- `415`: Content-Type is not application/x-www-form-urlencoded
- `500`: Database errors or other internal errors

## Security

- All user-provided content is HTML-escaped to prevent XSS attacks
- Form validation ensures all required fields are present
- URLs are validated before storage
- SQLite connection pool ensures safe concurrent access

## Data Types

### Post

A blog post consists of:
- `id`: Integer, auto-incrementing primary key
- `title`: String, required
- `body`: String, required
- `image_url`: String (valid URL), required

## Rate Limiting

Currently, there are no rate limits implemented on the API endpoints.

## Future Enhancements

Planned API improvements:
- JSON API endpoints for programmatic access
- Authentication and authorization
- Post deletion and updating
- Comment system
- Rate limiting
