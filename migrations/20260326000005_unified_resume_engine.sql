-- Explicit definition of the Rust Enum in PostgreSQL via ENUM Type
CREATE TYPE resume_category_enum AS ENUM (
    'work', 
    'education', 
    'skill', 
    'project', 
    'language', 
    'volunteer', 
    'extracurricular', 
    'hobby'
);

-- Upgrade `resume_profiles` schema natively
ALTER TABLE resume_profiles 
    ADD COLUMN target_role VARCHAR(255),
    ADD COLUMN contact_email VARCHAR(255),
    ADD COLUMN contact_phone VARCHAR(255),
    ADD COLUMN contact_location VARCHAR(255),
    ADD COLUMN contact_link VARCHAR(255),
    ADD COLUMN category_visibility JSONB DEFAULT '{}'::jsonb,
    RENAME COLUMN biography TO objective;

-- Drop archaic array relationship
DROP TABLE IF EXISTS resume_profile_items CASCADE;

-- Spin up Unified Database Architecture
CREATE TABLE resume_entries (
    id SERIAL PRIMARY KEY,
    profile_id INTEGER NOT NULL REFERENCES resume_profiles(id) ON DELETE CASCADE,
    category VARCHAR(255) NOT NULL,
    title VARCHAR(500) NOT NULL,
    subtitle VARCHAR(500),
    date_range VARCHAR(255),
    bullets JSONB NOT NULL DEFAULT '[]'::jsonb,
    display_order INTEGER NOT NULL DEFAULT 0,
    is_visible BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
