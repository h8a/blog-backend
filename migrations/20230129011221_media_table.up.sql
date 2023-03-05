-- Add up migration script here
CREATE TABLE IF NOT EXISTS media (
    id SERIAL PRIMARY KEY,
    name VARCHAR (255) NOT NULL,
    name_generated uuid NOT NULL,
    path VARCHAR (255) NOT NULL,
    content_type VARCHAR (255) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT NOW()
);