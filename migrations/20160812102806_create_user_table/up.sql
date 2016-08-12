CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY NOT NULL,
    email           varchar NOT NULL,
    password_hash   varchar NOT NULL,
    name            varchar NOT NULL
);
