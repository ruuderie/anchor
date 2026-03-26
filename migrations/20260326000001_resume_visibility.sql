-- Add is_public boolean to resume_profiles
ALTER TABLE resume_profiles ADD COLUMN is_public BOOLEAN NOT NULL DEFAULT FALSE;

-- Automatically set the primary profile (id = 1) to public if it exists
UPDATE resume_profiles SET is_public = TRUE WHERE id = 1;
