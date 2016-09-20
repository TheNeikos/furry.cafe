CREATE TABLE invites (
    id              BIGSERIAL PRIMARY KEY NOT NULL,
    user_id         BIGINT references users(id),
    created_at      timestamp NOT NULL default now(),
    updated_at      timestamp NOT NULL default now(),

    invite_key      varchar(50) NOT NULL
);

