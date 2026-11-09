-- Add migration script here
ALTER TABLE transactions ADD COLUMN currency TEXT NOT NULL DEFAULT 'CAD';