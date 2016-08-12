CREATE TABLE sessions (
    id          BIGSERIAL PRIMARY KEY NOT NULL,
    user_id     BIGINT references users(id) NOT NULL,
    created_at  timestamp NOT NULL default now(),
    updated_at  timestamp NOT NULL default now()
);
