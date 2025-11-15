CREATE TABLE IF NOT EXISTS urls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    long_url TEXT NOT NULL,
    short_url VARCHAR(20) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    click_count INTEGER NOT NULL DEFAULT 0
);

-- index for fast lookups
CREATE INDEX idx_urls_short_code ON urls(short_code);

-- index for descending sorting
CREATE INDEX idx_urls_created_at ON urls(created_at DESC);