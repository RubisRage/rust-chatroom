-- Add migration script here

CREATE TABLE IF NOT EXISTS chatrooms (
    name VARCHAR(255) PRIMARY KEY,
    password VARCHAR(255)
);

-- Example chatroom
INSERT OR IGNORE INTO chatrooms (name, password) VALUES ('example', 'example');
