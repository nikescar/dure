-- Drop DNS cache table
DROP INDEX IF EXISTS idx_dns_cache_timestamp;
DROP INDEX IF EXISTS idx_dns_cache_domain;
DROP TABLE IF EXISTS dns_cache;
