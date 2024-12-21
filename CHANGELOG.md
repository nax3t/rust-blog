# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Edit functionality for blog posts
  - Added routes for editing and updating posts
  - Created edit form template with validation
  - Added tests for edit functionality
- Create new post functionality with form validation
- Form validation error handling and display
- Support for image URL validation in new posts

### Fixed
- Route ordering issue where `/posts/new` was being caught by `/posts/<id>`
- Invalid post ID handling to return 404 instead of 422
- Form field validation in tests to match actual HTML structure

## [0.2.0] - 2024-12-20
### Added
- Complete migration from Axum to Rocket web framework
  - Restructured to MVC architecture
  - Moved templates to Tera template engine
  - Added form validation and error handling
  - Support for image URL validation
- CRUD operations for blog posts
  - Create: New post form with validation
  - Read: Post listing and detail views
  - Update: Edit form with validation
  - Delete: Confirmation dialog and proper redirection
- SQLite AUTOINCREMENT for post IDs
  - IDs now increment sequentially like Rails
  - Deleted post IDs are not reused
- Comprehensive test coverage
  - Unit tests for database operations
  - Integration tests for web routes
  - Validation and error handling tests

### Changed
- Enhanced routing to follow RESTful conventions
- Improved error handling for malformed data
- Better HTML escaping and URL sanitization
- Cleaner codebase structure
- Removed all Axum-related code and dependencies

### Fixed
- Route ordering for `/posts/new` and `/posts/<id>`
- Invalid post ID handling with proper 404s
- Form field validation in tests
- HTML escaping implementation

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
