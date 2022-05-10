-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON concepts;
DROP TABLE IF EXISTS concepts;