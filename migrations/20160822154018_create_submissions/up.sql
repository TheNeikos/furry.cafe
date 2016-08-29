CREATE TABLE submissions (
    id              BIGSERIAL PRIMARY KEY NOT NULL,
    user_id         BIGINT references users(id) NOT NULL,
    created_at      timestamp NOT NULL default now(),
    updated_at      timestamp NOT NULL default now(),

    image           BIGINT references images(id) NOT NULL,
    title           varchar(50) NOT NULL,
    description     varchar(150000) NOT NULL
);
