-- Your SQL goes here
CREATE TABLE access_tokens (
  id SERIAL PRIMARY KEY,
  user_id SERIAL REFERENCES users(id),
  token TEXT NOT NULL,
  valid_until TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);