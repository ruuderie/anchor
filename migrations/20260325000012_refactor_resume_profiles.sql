ALTER TABLE resume_profiles DROP COLUMN IF EXISTS excluded_tags;
ALTER TABLE resume_profiles DROP COLUMN IF EXISTS anonymous_companies;

CREATE TABLE IF NOT EXISTS resume_profile_items (
    profile_id INTEGER NOT NULL REFERENCES resume_profiles(id) ON DELETE CASCADE,
    item_type VARCHAR(50) NOT NULL,
    item_id INTEGER NOT NULL,
    custom_name VARCHAR(255),
    PRIMARY KEY (profile_id, item_type, item_id)
);
