-- Update ACME certificates table for lego compatibility
-- Replace ca_path and fullchain_path with issuer_path

-- Create new table with lego schema
CREATE TABLE IF NOT EXISTS acme_certificates_new (
    domain TEXT PRIMARY KEY,
    cert_path TEXT NOT NULL,
    key_path TEXT NOT NULL,
    issuer_path TEXT NOT NULL,
    issued_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    is_valid INTEGER NOT NULL
);

-- Copy data from old table, mapping ca_path to issuer_path
INSERT INTO acme_certificates_new (domain, cert_path, key_path, issuer_path, issued_at, expires_at, is_valid)
SELECT domain, cert_path, key_path, ca_path, issued_at, expires_at, is_valid
FROM acme_certificates;

-- Drop old table
DROP TABLE acme_certificates;

-- Rename new table
ALTER TABLE acme_certificates_new RENAME TO acme_certificates;

-- Recreate index
CREATE INDEX IF NOT EXISTS idx_acme_expires_at ON acme_certificates(expires_at);
