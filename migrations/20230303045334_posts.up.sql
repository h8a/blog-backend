-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    body TEXT NOT NULL,
    slug VARCHAR (510) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id INT,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS posts_references (
    id SERIAL PRIMARY KEY,
    name VARCHAR (510) NOT NULL,
    url TEXT,
    created_on TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    post_id INT,
    user_id INT,
    CONSTRAINT fk_post FOREIGN KEY(post_id) REFERENCES posts(id),
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
);