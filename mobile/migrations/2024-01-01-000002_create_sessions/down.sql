-- Drop sessions table
DROP INDEX IF EXISTS idx_sessions_last_seen;
DROP INDEX IF EXISTS idx_sessions_type;
DROP INDEX IF EXISTS idx_sessions_domain;
DROP TABLE IF EXISTS sessions;
