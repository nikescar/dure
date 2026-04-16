-- Create sites table for site-to-site communication
CREATE TABLE IF NOT EXISTS sites (
    domain TEXT PRIMARY KEY,
    public_key TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'disconnected',
    last_seen INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_sites_status ON sites(status);
CREATE INDEX IF NOT EXISTS idx_sites_last_seen ON sites(last_seen);
