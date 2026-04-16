-- Create NFT whitelist table
CREATE TABLE IF NOT EXISTS nft_whitelist (
    ip TEXT PRIMARY KEY,
    description TEXT NOT NULL,
    added_at INTEGER NOT NULL
);

-- Create index for recent additions
CREATE INDEX IF NOT EXISTS idx_nft_whitelist_added_at ON nft_whitelist(added_at);
