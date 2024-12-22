-- Add down migration script here

-- Drop all tables in reverse order
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;
