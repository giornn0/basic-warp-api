-- Your SQL goes here
CREATE TABLE concepts (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  amount NUMERIC NOT NULL,
  image TEXT,
  user_id SERIAL REFERENCES users(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON concepts
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();