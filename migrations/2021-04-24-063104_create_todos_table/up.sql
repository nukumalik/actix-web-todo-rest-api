-- Your SQL goes here
CREATE TABLE todos (
    id INTEGER PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    is_complete BOOLEAN NOT NULL DEFAULT 'f'
);