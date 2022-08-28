-- Your SQL goes here
CREATE TABLE pd_events (
    id SERIAL PRIMARY KEY,
    description VARCHAR NOT NULL,
    additional_info VARCHAR NOT NULL
)