# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Migration from Axum to Rocket web framework
- Restructuring to MVC architecture
- Moving templates to Tera template engine
- Create new post functionality with form validation
- Form validation error handling and display
- Support for image URL validation in new posts

### Fixed
- Route ordering issue where `/posts/new` was being caught by `/posts/<id>`
- Invalid post ID handling to return 404 instead of 422
- Form field validation in tests to match actual HTML structure

## [0.2.0] - Unreleased
### Added
- Delete functionality for blog posts
  - Method override support for DELETE operations
  - Confirmation dialog before deletion
  - Proper form handling and redirection
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
- Edit button in show post view
- Method override middleware for handling PUT requests from HTML forms

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
- Improved post ID handling in database operations
- Simplified form submission by removing redundant JavaScript
- Added proper HTML escaping in form values

### Fixed
- Corrected status code expectations for malformed content types
- Fixed 405 error when submitting edit form by adding POST route that transforms to PUT

## [0.1.0] - 2024-12-20
### Added
- Complete CRUD operations for blog posts
  - Create new posts with title, body, and image URL
  - Read posts with list and detail views
  - Update existing posts
  - Delete posts with confirmation
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
  - API Documentation
  - Development Guide

### Security
- HTML escaping for XSS prevention
- URL sanitization for image URLs
- Form validation and sanitization
- Error handling for malformed input
