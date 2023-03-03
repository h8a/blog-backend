-- Add up migration script here
ALTER TABLE media
ADD COLUMN user_id INT;

ALTER TABLE media
ADD CONSTRAINT fk_user
FOREIGN KEY (user_id)
REFERENCES users(id);