-- Add down migration script here
ALTER TABLE media
DROP COLUMN IF EXISTS user_id ON DELETE CASCADE;