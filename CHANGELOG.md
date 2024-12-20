# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Web server implementation using Axum
- Thread-safe SQLite connection pool using r2d2
- Web routes for viewing and creating posts
- Integration tests for web endpoints
- Manual testing capability via localhost:3000
- HTML escaping for XSS prevention
- Comprehensive test suite for database and web functionality
  - Empty database tests
  - Post order tests
  - Malformed data handling
  - HTML escaping tests
- Comprehensive documentation in docs/ directory
  - Quick Start Guide
  - API Overview and Endpoints
  - Development Guide with Best Practices
  - Database Schema Documentation
  - Installation and Configuration Guides
- Post detail view feature
  - View single post by ID with formatted paragraphs
  - Handle non-existent posts with 404 response
  - Validate post ID format with 400 response
  - Links to individual posts from index page
  - Back to Posts navigation
- Post editing feature with RESTful PUT endpoint
  - Edit form with HTML escaping and URL sanitization
  - Method override support for HTML forms
  - Comprehensive test coverage including validation and XSS prevention

### Changed
- Enhanced routing to follow RESTful conventions
- Added proper form validation with UNPROCESSABLE_ENTITY status codes
- Added Post ID getter and usage in HTML output
- Fixed compiler warnings for unused imports and fields
- Improved error handling for malformed form data
- Updated test expectations for content type validation
- Fixed HTML escaping implementation in index handler
- Enhanced project documentation structure and organization
- Improved post list view with clickable titles

### Fixed
- Corrected status code expectations for malformed content types

## [0.1.0] - 2024-12-19
### Added
- Initial project setup with Cargo
- Basic Post struct with title, body, and image_url fields
- SQLite database integration using rusqlite
- Basic CRUD operations (create and read) for blog posts
- Integration tests for post creation and persistence
