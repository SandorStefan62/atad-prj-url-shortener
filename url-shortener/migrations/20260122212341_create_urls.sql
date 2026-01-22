CREATE TABLE IF NOT EXISTS urls (
    id UUID PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    original_url TEXT NOT NULL,
    short_code TEXT UNIQUE NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT,
    click_count INTEGER NOT NULL DEFAULT 0
);

-- index for fast lookups
CREATE INDEX IF NOT EXISTS idx_urls_short_code ON urls(short_code);

-- index for descending sorting
CREATE INDEX IF NOT EXISTS idx_urls_created_at ON urls(created_at DESC);