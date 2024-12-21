# API Overview

## Introduction

The Rust Blog is built using the Rocket web framework, providing a RESTful API for managing blog posts. The application uses SQLite for data storage and Tera for templating.

## Routes

### Index
- Path: `GET /`
- Description: Redirects to the posts index
- Response: 302 redirect to `/posts`

### List Posts
- Path: `GET /posts`
- Description: Display all blog posts
- Template: `posts/index.html.tera`
- Context:
  ```rust
  {
    title: String,
    posts: Vec<Post>
  }
  ```

### New Post Form
- Path: `GET /posts/new`
- Description: Display the new post form
- Template: `posts/new.html.tera`
- Context:
  ```rust
  {
    title: String,
    error?: String,
    form?: PostForm
  }
  ```

### Create Post
- Path: `POST /posts`
- Description: Create a new blog post
- Form Parameters:
  - `title`: String (required)
  - `body`: String (required)
  - `image_url`: String (required, must be valid URL)
- Response:
  - Success: 302 redirect to `/posts/<id>`
  - Error: 422 with form and error message

### Show Post
- Path: `GET /posts/:id`
- Description: Display a single post
- Parameters:
  - `id`: i64 (post ID)
- Template: `posts/show.html.tera`
- Context:
  ```rust
  {
    title: String,
    post: Post
  }
  ```
- Response:
  - Success: 200 with post
  - Not Found: 404

### Edit Post Form
- Path: `GET /posts/:id/edit`
- Description: Display the edit form for a post
- Parameters:
  - `id`: i64 (post ID)
- Template: `posts/edit.html.tera`
- Context:
  ```rust
  {
    title: String,
    post: Post,
    error?: String,
    form?: PostForm
  }
  ```
- Response:
  - Success: 200 with form
  - Not Found: 404

### Update Post
- Path: `PUT /posts/:id`
- Description: Update an existing post
- Parameters:
  - `id`: i64 (post ID)
- Form Parameters:
  - `title`: String (required)
  - `body`: String (required)
  - `image_url`: String (required, must be valid URL)
  - `_method`: "PUT" (for method override)
- Response:
  - Success: 302 redirect to `/posts/<id>`
  - Error: 422 with form and error message
  - Not Found: 404

### Delete Post
- Path: `DELETE /posts/:id`
- Description: Delete a post
- Parameters:
  - `id`: i64 (post ID)
- Form Parameters:
  - `_method`: "DELETE" (for method override)
- Response:
  - Success: 302 redirect to `/posts`
  - Not Found: 404

## Data Types

### Post
```rust
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub body: String,
    pub image_url: String,
}
```

### PostForm
```rust
pub struct PostForm {
    pub title: String,
    pub body: String,
    pub image_url: String,
    pub _method: Option<String>,
}
```

## Error Handling

The application handles several types of errors:

### 404 Not Found
- When a post ID doesn't exist
- Custom error template: `error/404.html.tera`

### 422 Unprocessable Entity
- Invalid form data
- Missing required fields
- Invalid image URL
- Custom error template with form preservation

### 500 Internal Server Error
- Database errors
- Template rendering errors
- Custom error template: `error/500.html.tera`

## Templates

The application uses Tera templates stored in the `templates` directory:

```
templates/
├── base.html.tera
├── error/
│   ├── 404.html.tera
│   └── 500.html.tera
└── posts/
    ├── edit.html.tera
    ├── index.html.tera
    ├── new.html.tera
    └── show.html.tera
```

## Security Features

1. **CSRF Protection**
   - Method override tokens for PUT/DELETE
   - Form validation

2. **XSS Prevention**
   - HTML escaping in templates
   - URL sanitization for images

3. **Input Validation**
   - Required field checking
   - URL format validation
   - Error messages for invalid input
