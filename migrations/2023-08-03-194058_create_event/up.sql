-- Your SQL goes here
create table events (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    event_time timestamp with time zone NOT NULL,
    created_time timestamp with time zone default current_timestamp
)
