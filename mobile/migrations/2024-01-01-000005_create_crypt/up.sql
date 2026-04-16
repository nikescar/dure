-- Create cryptographic keys table
CREATE TABLE IF NOT EXISTS crypt_keys (
    device_id TEXT PRIMARY KEY,
    private_key BLOB NOT NULL,
    public_key BLOB NOT NULL,
    created_at INTEGER NOT NULL
);

-- Create index for recent keys
CREATE INDEX IF NOT EXISTS idx_crypt_keys_created_at ON crypt_keys(created_at DESC);
