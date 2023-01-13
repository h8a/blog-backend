-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR (255) NOT NULL,
    password VARCHAR (255) NOT NULL,
    name VARCHAR (255) NOT NULL,
    last_name VARCHAR (255) NOT NULL,
    surname VARCHAR (255) NULL,
    picture VARCHAR (255) NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);