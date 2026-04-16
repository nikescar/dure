-- Drop webhook tables
DROP INDEX IF EXISTS idx_webhook_requests_received_at;
DROP INDEX IF EXISTS idx_webhook_requests_pattern;
DROP TABLE IF EXISTS webhook_requests;
DROP TABLE IF EXISTS webhook_allow_patterns;
DROP TABLE IF EXISTS webhook_config;
