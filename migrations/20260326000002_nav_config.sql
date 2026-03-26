CREATE TABLE IF NOT EXISTS nav_items (
    id SERIAL PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    href VARCHAR(255),
    parent_id INTEGER REFERENCES nav_items(id) ON DELETE CASCADE,
    display_order INTEGER NOT NULL DEFAULT 0,
    is_visible BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO nav_items (label, href, display_order) VALUES
('WORK', '/work', 10),
('PROJECTS', '/projects', 20),
('BLOG', '/blog', 30),
('REAL ESTATE', '/real-estate', 40);
