-- This file should undo anything in `up.sql`
ALTER TABLE events
DROP COLUMN domain;

ALTER TABLE events
DROP COLUMN owner;
