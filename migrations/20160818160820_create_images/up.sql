CREATE TABLE images (
    id              BIGSERIAL PRIMARY KEY NOT NULL,
    created_at      timestamp NOT NULL default now(),
    updated_at      timestamp NOT NULL default now(),
    host_type       integer NOT NULL,
    path            varchar NOT NULL
);
