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
- `303 See Other`: Successfully created, redirects to post view
- `422 Unprocessable Entity`: Invalid input data
- `500 Internal Server Error`: Server error occurred

### View Post
```
GET /posts/:id
```

Returns the HTML page for viewing a specific post.

**Parameters**:
- `id`: The post ID (integer)

**Response**:
- `200 OK`: Successfully retrieved post
- `404 Not Found`: Post not found
- `400 Bad Request`: Invalid post ID format

### Edit Post Form
```
GET /posts/:id/edit
```

Returns the HTML form for editing an existing post.

**Parameters**:
- `id`: The post ID (integer)

**Response**:
- `200 OK`: Successfully retrieved form
- `404 Not Found`: Post not found
- `400 Bad Request`: Invalid post ID format

### Update Post
```
PUT /posts/:id
```

Updates an existing post. Note: HTML forms use POST with method override.

**Parameters**:
- `id`: The post ID (integer)

**Request Body** (form-urlencoded):
```
_method=PUT&title=Updated Title&body=Updated Content&image_url=https://example.com/new.jpg
```

**Fields**:
- `_method` (required): Must be "PUT" for method override
- `title` (required): The updated post title
- `body` (required): The updated post content
- `image_url` (required): Valid URL to an image

**Response**:
- `303 See Other`: Successfully updated, redirects to post view
- `404 Not Found`: Post not found
- `400 Bad Request`: Invalid post ID format
- `422 Unprocessable Entity`: Invalid input data
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

## Security Features

### XSS Prevention
- All HTML output is properly escaped
- URLs are sanitized to prevent dangerous protocols (javascript:, data:, vbscript:)

### Method Override Security
- PUT requests are handled via POST with method override
- Method override is only accepted from form submissions

### Input Validation
- All form inputs are validated before processing
- URLs are validated for format and safety
- Post IDs are validated as integers

## Data Types

### Post

A blog post consists of:
- `id`: Integer, auto-incrementing primary key
- `title`: String, required
- `body`: String, required
- `image_url`: String (valid URL), required

## Future Enhancements

The following features are planned for future releases:
- JSON API endpoints for programmatic access
- Authentication and authorization
- Post deletion
- Categories and tags
- User authentication
- Comments system
- Rate limiting

## Rate Limiting

Currently, there are no rate limits implemented on the API endpoints.
