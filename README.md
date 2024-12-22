# Rust Blog Application

A modern, secure blogging platform built with Rust, Rocket.rs, and Tailwind CSS. This application provides a full-featured blogging experience with user authentication, post management, and commenting system.

## Features

  - **User Authentication**
    - Secure user registration and login
    - Password hashing using bcrypt
    - Session management with secure cookies
    - Protected routes for authenticated users

  - **Blog Posts**
    - Create, read, update, and delete blog posts
    - Clean typography with Tailwind's prose styling
    - Author attribution
    - Timestamp tracking for creation and updates

  - **Comments**
    - Comment on blog posts
    - Edit and delete your own comments
    - Author information and timestamps
    - Chronological display

  - **Modern UI**
    - Responsive design using Tailwind CSS
    - Clean and intuitive interface
    - Consistent styling across all pages
    - Mobile-friendly layout

## Technology Stack

  - **Backend**
    - Rust
    - Rocket.rs web framework
    - SQLite database
    - Rusqlite for database operations
    - Bcrypt for password hashing
    - Tera templating engine

  - **Frontend**
    - Tailwind CSS for styling
    - HTML templates with Tera
    - Responsive design
    - Modern form handling

## Project Structure

```
blog/
├── src/
│   ├── main.rs              # Application entry point and server setup
│   ├── auth.rs              # Authentication logic and middleware
│   ├── models/              # Data models and structures
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── post.rs
│   │   └── comment.rs
│   ├── routes/              # Route handlers
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── posts.rs
│   │   └── comments.rs
│   └── services/           # Business logic and database operations
│       ├── mod.rs
│       ├── db.rs
│       ├── user_service.rs
│       ├── post_service.rs
│       └── comment_service.rs
├── templates/              # Tera HTML templates
│   ├── base.html.tera
│   ├── index.html.tera
│   ├── login.html.tera
│   ├── register.html.tera
│   └── post.html.tera
├── static/                # Static assets
│   └── assets/
│       └── css/
│           ├── input.css  # Tailwind source CSS
│           └── output.css # Compiled CSS
├── Cargo.toml            # Rust dependencies
└── package.json         # Node.js dependencies (for Tailwind)
```

## Setup and Installation

1. **Prerequisites**
   - Rust and Cargo (latest stable version)
   - Node.js and npm (for Tailwind CSS)
   - SQLite

2. **Installation**
   ```bash
   # Clone the repository
   git clone <repository-url>
   cd blog

   # Install Rust dependencies
   cargo build

   # Install Node.js dependencies
   npm install

   # Build CSS
   npm run build:css
   ```

3. **Database Setup**
   ```bash
   # Create and seed the SQLite database
   sqlite3 blog.db < seed.sql
   ```

   This will create the database with test users:
   - alice (password: password123)
   - bob (password: password123)
   - carol (password: password123)

   And populate it with sample posts and comments.

4. **Running the Application**
   ```bash
   # Start the development server
   cargo run

   # In a separate terminal, watch for CSS changes
   npm run watch:css
   ```

   The application will be available at `http://localhost:<port>` (check the console output for the exact port)

## Development

  - **Database**: Use the `schema.sql` file to set up your database structure
  - **CSS Changes**: Run `npm run watch:css` to automatically compile Tailwind CSS changes
  - **Templates**: Modify `.tera` files in the `templates` directory for UI changes
  - **Routing**: Add new routes in `src/routes` and register them in `main.rs`

## Security Features

  - Password hashing using bcrypt with appropriate cost factor
  - Secure session management with private cookies
  - Input validation and sanitization
  - Protected routes with authentication middleware

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
