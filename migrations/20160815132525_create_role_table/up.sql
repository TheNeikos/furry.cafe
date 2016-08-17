CREATE TABLE user_roles (
    id          BIGSERIAL PRIMARY KEY NOT NULL,
    user_id     BIGINT references users(id) NOT NULL,
    role        INT NOT NULL default 1,
    created_at  timestamp NOT NULL default now(),
    updated_at  timestamp NOT NULL default now()
);

CREATE UNIQUE INDEX user_role_id_unique_index on user_roles(user_id);
