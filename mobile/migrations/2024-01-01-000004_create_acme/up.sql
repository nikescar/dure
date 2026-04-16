-- Create ACME certificates table
CREATE TABLE IF NOT EXISTS acme_certificates (
    domain TEXT PRIMARY KEY,
    cert_path TEXT NOT NULL,
    key_path TEXT NOT NULL,
    ca_path TEXT NOT NULL,
    fullchain_path TEXT NOT NULL,
    issued_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    is_valid INTEGER NOT NULL
);

-- Create index for expiration checks
CREATE INDEX IF NOT EXISTS idx_acme_expires_at ON acme_certificates(expires_at);
