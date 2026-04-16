-- Create sessions table for HTTP and WSS connections
CREATE TABLE IF NOT EXISTS sessions (
    session_id TEXT PRIMARY KEY,
    domain TEXT NOT NULL,
    session_type TEXT NOT NULL,
    connected_at INTEGER NOT NULL,
    last_seen INTEGER NOT NULL,
    request_count INTEGER NOT NULL,
    remote_addr TEXT NOT NULL
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_sessions_domain ON sessions(domain);
CREATE INDEX IF NOT EXISTS idx_sessions_type ON sessions(session_type);
CREATE INDEX IF NOT EXISTS idx_sessions_last_seen ON sessions(last_seen);
