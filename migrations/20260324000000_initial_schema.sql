CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  passkey JSONB NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  session_token TEXT UNIQUE
);

CREATE TABLE auth_challenges (
  id UUID PRIMARY KEY,
  challenge_data JSONB NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE jobs (
  id SERIAL PRIMARY KEY,
  date_range TEXT NOT NULL,
  role TEXT NOT NULL,
  company TEXT NOT NULL,
  description TEXT NOT NULL,
  tags TEXT[] NOT NULL DEFAULT '{}'
);

CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  slug TEXT UNIQUE NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  image_url TEXT,
  tech_percent JSONB NOT NULL DEFAULT '{}',
  impact TEXT NOT NULL,
  tags TEXT[] NOT NULL DEFAULT '{}'
);

CREATE TABLE certifications (
  id SERIAL PRIMARY KEY,
  date_range TEXT NOT NULL,
  role TEXT NOT NULL,
  company TEXT NOT NULL,
  description TEXT NOT NULL,
  tags TEXT[] NOT NULL DEFAULT '{}'
);

CREATE TABLE blog_posts (
  id SERIAL PRIMARY KEY,
  slug TEXT UNIQUE NOT NULL,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  tags TEXT[] NOT NULL DEFAULT '{}'
);
