CREATE TABLE mailing_list (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    list_type TEXT NOT NULL,
    preferences JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);
