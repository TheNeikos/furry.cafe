CREATE TABLE unique_codes (
    id              BIGSERIAL PRIMARY KEY NOT NULL,
    user_id         BIGINT references users(id),
    created_at      timestamp NOT NULL default now(),
    updated_at      timestamp NOT NULL default now(),

    code            varchar(50) NOT NULL,
    typ             INT NOT NULL
);


