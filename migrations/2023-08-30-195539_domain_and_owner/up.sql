ALTER TABLE events
ADD COLUMN domain varchar;

ALTER TABLE events
add column owner bigint;
