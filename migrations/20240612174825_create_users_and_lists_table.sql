-- CREATE TABLE users (
--   id SERIAL PRIMARY KEY,
--   email VARCHAR(255) NOT NULL UNIQUE,
--   name VARCHAR(255) NOT NULL,
--   salt VARCHAR(255) NOT NULL,
--   password VARCHAR(255) NOT NULL,
--   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
--   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );
CREATE TABLE lists (
  id SERIAL PRIMARY KEY,
  -- created_by INTEGER NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  completed_at TIMESTAMP,
  -- completed_by INTEGER,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
  -- CONSTRAINT fk_created_by_user FOREIGN KEY (created_by) REFERENCES users (id),
  -- CONSTRAINT fk_completed_by_user FOREIGN KEY (completed_by) REFERENCES users (id)
);
