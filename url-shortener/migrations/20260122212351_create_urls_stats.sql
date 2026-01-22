CREATE TABLE IF NOT EXISTS clicks (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    url_id TEXT NOT NULL REFERENCES urls(id) ON DELETE CASCADE,
    clicked_at TEXT NOT NULL DEFAULT (datetime('now')),
    ip_address TEXT,
    user_agent TEXT,
    referer TEXT,
    country TEXT,
    city TEXT
);

-- Create indexes for analytics queries
CREATE INDEX IF NOT EXISTS idx_clicks_url_id ON clicks(url_id);
CREATE INDEX IF NOT EXISTS idx_clicks_clicked_at ON clicks(clicked_at DESC);
CREATE INDEX IF NOT EXISTS idx_clicks_ip_address ON clicks(ip_address);
CREATE INDEX IF NOT EXISTS idx_clicks_country ON clicks(country);