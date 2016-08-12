CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY NOT NULL,
    email           varchar NOT NULL,
    password_hash   varchar NOT NULL,
    name            varchar NOT NULL,
    created_at      timestamp NOT NULL default now(),
    updated_at      timestamp NOT NULL default now()
);
