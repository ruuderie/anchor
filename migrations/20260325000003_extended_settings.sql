INSERT INTO site_settings (key, value) VALUES 
    ('site_title', 'RUUDERIE_AI'),
    ('lead_capture_title', 'Request Tailored CV'),
    ('lead_capture_desc', 'Input your protocol for a mission-specific credentials package.'),
    ('lead_capture_label', 'Registry Email Address'),
    ('lead_capture_placeholder', 'user@organization.domain'),
    ('lead_capture_btn', 'Initialize Retrieval'),
    ('lead_capture_footer', '* Check your email to confirm the request parameters.'),
    ('lead_capture_endpoint', '/api/DownloadResume')
ON CONFLICT DO NOTHING;
