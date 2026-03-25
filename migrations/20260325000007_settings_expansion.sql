INSERT INTO site_settings (key, value) VALUES 
    ('webhook_url', ''),
    ('admin_email', ''),
    ('landing_options_json', '{"resume": "Request Tailored CV", "mailing_list": "Join Mailing List"}')
ON CONFLICT DO NOTHING;
