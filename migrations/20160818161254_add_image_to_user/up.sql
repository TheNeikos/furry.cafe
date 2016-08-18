ALTER TABLE users ADD COLUMN profile_image BIGINT references images(id);
