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
    author_username TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS comments (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    post_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    author_username TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (author_id) REFERENCES users(id)
);

-- Seed Data

-- Test Users (password is 'password123' for all users)
INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'alice', '$2b$12$61M8WS.UrAE69GJoea5JQeOJi7yJOmzvc8idfTNB2ceftaf4pYyq2', '2024-01-01', '2024-01-01'),
('550e8400-e29b-41d4-a716-446655440001', 'bob', '$2b$12$61M8WS.UrAE69GJoea5JQeOJi7yJOmzvc8idfTNB2ceftaf4pYyq2', '2024-01-01', '2024-01-01'),
('550e8400-e29b-41d4-a716-446655440002', 'carol', '$2b$12$61M8WS.UrAE69GJoea5JQeOJi7yJOmzvc8idfTNB2ceftaf4pYyq2', '2024-01-01', '2024-01-01');

-- Test Posts
INSERT INTO posts (id, title, content, author_id, author_username, created_at, updated_at) VALUES
('660e8400-e29b-41d4-a716-446655440000', 'Getting Started with Rust', 
'Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. 

Here are some key features:
- Zero-cost abstractions
- Move semantics
- Guaranteed memory safety
- Threads without data races
- Trait-based generics
- Pattern matching
- Type inference
- Minimal runtime
- Efficient C bindings',
'550e8400-e29b-41d4-a716-446655440000', 'alice', '2024-01-01', '2024-01-01'),

('660e8400-e29b-41d4-a716-446655440001', 'Web Development with Rocket.rs',
'Rocket is a web framework for Rust that makes it simple to write fast, secure web applications. It provides a great developer experience without sacrificing performance or safety.

Key concepts:
1. Request handling
2. Response generation
3. State management
4. Error handling
5. Testing

Stay tuned for more posts about Rocket.rs!',
'550e8400-e29b-41d4-a716-446655440001', 'bob', '2024-01-02', '2024-01-02'),

('660e8400-e29b-41d4-a716-446655440002', 'Modern CSS with Tailwind',
'Tailwind CSS is a utility-first CSS framework that can be composed to build any design, directly in your markup.

Benefits:
- Rapid development
- Consistent design
- Small bundle size
- Customizable
- Great documentation

Examples coming soon!',
'550e8400-e29b-41d4-a716-446655440002', 'carol', '2024-01-03', '2024-01-03');

-- Test Comments
INSERT INTO comments (id, content, post_id, author_id, author_username, created_at, updated_at) VALUES
('770e8400-e29b-41d4-a716-446655440000', 
'Great introduction to Rust! The memory safety features are particularly impressive.',
'660e8400-e29b-41d4-a716-446655440000', 
'550e8400-e29b-41d4-a716-446655440001', 'bob', 
'2024-01-01', '2024-01-01'),

('770e8400-e29b-41d4-a716-446655440001', 
'I''ve been using Rocket for a while now, and it''s amazing how productive you can be with it.',
'660e8400-e29b-41d4-a716-446655440001', 
'550e8400-e29b-41d4-a716-446655440002', 'carol', 
'2024-01-02', '2024-01-02'),

('770e8400-e29b-41d4-a716-446655440002', 
'Tailwind has transformed how I write CSS. No more fighting with specificity!',
'660e8400-e29b-41d4-a716-446655440002', 
'550e8400-e29b-41d4-a716-446655440000', 'alice', 
'2024-01-03', '2024-01-03');
