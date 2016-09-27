ALTER TABLE images ADD COLUMN parent_id BIGINT references images(id);

