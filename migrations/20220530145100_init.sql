-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    id SERIAL,
    username VARCHAR(30) NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL,
    PRIMARY KEY (id)
);