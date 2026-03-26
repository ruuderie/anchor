CREATE TABLE IF NOT EXISTS landing_pages (
    id SERIAL PRIMARY KEY,
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    hero_title VARCHAR(255) NOT NULL,
    hero_subtitle VARCHAR(255) NOT NULL,
    lead_capture_title VARCHAR(255) NOT NULL,
    lead_capture_desc TEXT NOT NULL,
    lead_capture_btn VARCHAR(50) NOT NULL,
    options_json TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
