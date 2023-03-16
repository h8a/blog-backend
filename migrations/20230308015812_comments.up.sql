-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts_comments (
    id SERIAL PRIMARY KEY,
    comment TEXT NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    nickname VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    parent_id INT REFERENCES posts_comments(id),
    post_id INT,
    CONSTRAINT fk_post FOREIGN KEY(post_id) REFERENCES posts(id)
);