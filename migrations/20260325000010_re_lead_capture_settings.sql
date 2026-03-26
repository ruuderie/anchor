INSERT INTO site_settings (key, value)
VALUES 
    ('re_lc_title', 'Let''s Connect'),
    ('re_lc_desc', 'Join the deal flow or request financing. Select your areas of interest below.'),
    ('re_lc_label', 'Registry Email Address'),
    ('re_lc_placeholder', 'investor@domain.com'),
    ('re_lc_btn', 'SUBMIT INQUIRY'),
    ('re_options_json', '{"buying": "Buying a Home", "selling": "Selling a Home", "loan": "Getting a real estate investment loan", "networking": "Connecting with other investors"}')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;
