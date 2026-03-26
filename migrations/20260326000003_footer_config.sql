CREATE TABLE IF NOT EXISTS footer_items (
    id SERIAL PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    href VARCHAR(255),
    display_order INTEGER NOT NULL DEFAULT 0,
    is_visible BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO footer_items (label, href, display_order) VALUES
('TERMS OF SERVICE', '/terms', 10),
('PRIVACY POLICY', '/privacy', 20),
('SITEMAP', '/sitemap', 30);
