-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS trigger_set_updated_at();
DROP TRIGGER set_timestamp ON users;
DROP TABLE IF EXISTS users;
