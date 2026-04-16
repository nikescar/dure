-- Drop ACME certificates table
DROP INDEX IF EXISTS idx_acme_expires_at;
DROP TABLE IF EXISTS acme_certificates;
