-- Create DNS cache table
CREATE TABLE IF NOT EXISTS dns_cache (
    domain TEXT NOT NULL,
    record_type TEXT NOT NULL,
    value TEXT NOT NULL,
    ttl INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    PRIMARY KEY (domain, record_type, value)
);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_dns_cache_domain ON dns_cache(domain);
CREATE INDEX IF NOT EXISTS idx_dns_cache_timestamp ON dns_cache(timestamp);
