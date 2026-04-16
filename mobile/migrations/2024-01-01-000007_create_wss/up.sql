-- Create WebSocket server configurations table
CREATE TABLE IF NOT EXISTS wss_servers (
    domain TEXT PRIMARY KEY,
    bind_addr TEXT NOT NULL,
    bind_port INTEGER NOT NULL,
    server_id TEXT NOT NULL,
    ping_interval INTEGER NOT NULL,
    idle_timeout INTEGER NOT NULL,
    max_connections INTEGER NOT NULL
);

-- Create WebSocket sessions table
CREATE TABLE IF NOT EXISTS wss_sessions (
    session_id TEXT PRIMARY KEY,
    domain TEXT NOT NULL,
    connected_at INTEGER NOT NULL,
    last_seen INTEGER NOT NULL,
    message_count INTEGER NOT NULL,
    reconnect_count INTEGER NOT NULL,
    FOREIGN KEY (domain) REFERENCES wss_servers(domain) ON DELETE CASCADE
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_wss_sessions_domain ON wss_sessions(domain);
CREATE INDEX IF NOT EXISTS idx_wss_sessions_last_seen ON wss_sessions(last_seen);
