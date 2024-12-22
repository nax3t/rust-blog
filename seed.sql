-- Schema
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS posts (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    author_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS comments (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    post_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users(id)
);

-- Seed Data

-- Test Users (password is 'password123' for all users)
INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'alice', '$2b$12$61M8WS.UrAE69GJoea5JQeOJi7yJOmzvc8idfTNB2ceftaf4pYyq2', '2024-01-01', '2024-01-01'),
('550e8400-e29b-41d4-a716-446655440001', 'bob', '$2b$12$61M8WS.UrAE69GJoea5JQeOJi7yJOmzvc8idfTNB2ceftaf4pYyq2', '2024-01-01', '2024-01-01'),
('550e8400-e29b-41d4-a716-446655440002', 'carol', '$2b$12$61M8WS.UrAE69GJoea5JQeOJi7yJOmzvc8idfTNB2ceftaf4pYyq2', '2024-01-01', '2024-01-01');

-- Test Posts
INSERT INTO posts (id, title, content, author_id, created_at, updated_at) VALUES
('660e8400-e29b-41d4-a716-446655440000', 'Getting Started with Rust', 
'Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. 

Here are some key features:
- Zero-cost abstractions
- Move semantics
- Guaranteed memory safety
- Threads without data races
- Trait-based generics

Let''s start with a simple example...', 
'550e8400-e29b-41d4-a716-446655440000', '2024-01-01', '2024-01-01'),

('660e8400-e29b-41d4-a716-446655440001', 'Web Development with Rocket', 
'Rocket is a web framework for Rust that makes it simple to write fast, secure web applications.

Key features include:
- Type-safe routing
- Form handling
- JSON support
- Static file serving
- Template support

Here''s how to get started...', 
'550e8400-e29b-41d4-a716-446655440001', '2024-01-01', '2024-01-01');

-- Test Comments
INSERT INTO comments (id, content, post_id, author_id, created_at, updated_at) VALUES
('770e8400-e29b-41d4-a716-446655440000', 'Great introduction! Looking forward to learning more.', 
'660e8400-e29b-41d4-a716-446655440000', '550e8400-e29b-41d4-a716-446655440001', '2024-01-01', '2024-01-01'),

('770e8400-e29b-41d4-a716-446655440001', 'Thanks for sharing! The examples are very helpful.', 
'660e8400-e29b-41d4-a716-446655440000', '550e8400-e29b-41d4-a716-446655440002', '2024-01-01', '2024-01-01'),

('770e8400-e29b-41d4-a716-446655440002', 'Rocket looks promising. Have you tried comparing it with Actix?', 
'660e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440000', '2024-01-01', '2024-01-01');
