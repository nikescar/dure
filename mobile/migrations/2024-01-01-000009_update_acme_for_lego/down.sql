-- Revert lego schema changes back to acme.sh
CREATE TABLE IF NOT EXISTS acme_certificates_old (
    domain TEXT PRIMARY KEY,
    cert_path TEXT NOT NULL,
    key_path TEXT NOT NULL,
    ca_path TEXT NOT NULL,
    fullchain_path TEXT NOT NULL,
    issued_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    is_valid INTEGER NOT NULL
);

-- Copy data, mapping issuer_path back to ca_path and duplicating for fullchain_path
INSERT INTO acme_certificates_old (domain, cert_path, key_path, ca_path, fullchain_path, issued_at, expires_at, is_valid)
SELECT domain, cert_path, key_path, issuer_path, issuer_path, issued_at, expires_at, is_valid
FROM acme_certificates;

DROP TABLE acme_certificates;
ALTER TABLE acme_certificates_old RENAME TO acme_certificates;
CREATE INDEX IF NOT EXISTS idx_acme_expires_at ON acme_certificates(expires_at);
