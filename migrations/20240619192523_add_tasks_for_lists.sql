CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  list_id INTEGER NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  due_date TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT NOW (),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
  FOREIGN KEY (list_id) REFERENCES lists (id)
)
