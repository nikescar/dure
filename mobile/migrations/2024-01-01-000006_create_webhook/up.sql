-- Create webhook configuration table
CREATE TABLE IF NOT EXISTS webhook_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    logging_enabled INTEGER NOT NULL DEFAULT 0
);

-- Insert default config
INSERT OR IGNORE INTO webhook_config (id, logging_enabled) VALUES (1, 0);

-- Create webhook allow patterns table
CREATE TABLE IF NOT EXISTS webhook_allow_patterns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL
);

-- Create webhook requests table
CREATE TABLE IF NOT EXISTS webhook_requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern TEXT NOT NULL,
    path TEXT NOT NULL,
    method TEXT NOT NULL,
    headers TEXT NOT NULL,
    body TEXT NOT NULL,
    remote_addr TEXT NOT NULL,
    received_at INTEGER NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_webhook_requests_pattern ON webhook_requests(pattern);
CREATE INDEX IF NOT EXISTS idx_webhook_requests_received_at ON webhook_requests(received_at);
