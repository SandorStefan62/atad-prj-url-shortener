CREATE TABLE IF NOT EXISTS stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url_id UUID NOT NULL REFERENCES urls(id) ON DELETE CASCADE,
    clicked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address VARCHAR(45),
    user_agent TEXT,
    referer TEXT,
    country VARCHAR(100),
    city VARCHAR(100)
);

-- indexes for fast analytics queries
CREATE INDEX IF NOT EXISTS idx_clicks_url_id ON stats(url_id);
CREATE INDEX IF NOT EXISTS idx_clicks_clicked_at ON stats(clicked_at DESC);
CREATE INDEX IF NOT EXISTS idx_clicks_ip_address ON stats(ip_address);
CREATE INDEX IF NOT EXISTS idx_clicks_coujntry ON stats(country);