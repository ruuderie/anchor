CREATE TABLE IF NOT EXISTS site_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT INTO site_settings (key, value) VALUES 
    ('current_focus', 'AI Agent Swarms (Agentforce / CrewAI)'),
    ('status', 'Available for Critical Ops')
ON CONFLICT DO NOTHING;

ALTER TABLE projects ADD COLUMN IF NOT EXISTS status TEXT DEFAULT 'Completed';
UPDATE projects SET status = 'Completed' WHERE status IS NULL;
