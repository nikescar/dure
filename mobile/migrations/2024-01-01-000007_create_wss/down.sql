-- Drop WebSocket tables
DROP INDEX IF EXISTS idx_wss_sessions_last_seen;
DROP INDEX IF EXISTS idx_wss_sessions_domain;
DROP TABLE IF EXISTS wss_sessions;
DROP TABLE IF EXISTS wss_servers;
