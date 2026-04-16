-- Create authenticated_devices table for storing device authentication information
CREATE TABLE IF NOT EXISTS authenticated_devices (
    device_id TEXT PRIMARY KEY NOT NULL,
    public_key TEXT NOT NULL,
    session_id TEXT NOT NULL,
    authenticated_at INTEGER NOT NULL,
    last_seen INTEGER NOT NULL
);

-- Index for fast lookups by session_id
CREATE INDEX IF NOT EXISTS idx_authenticated_devices_session_id
    ON authenticated_devices(session_id);

-- Index for finding stale devices
CREATE INDEX IF NOT EXISTS idx_authenticated_devices_last_seen
    ON authenticated_devices(last_seen);
