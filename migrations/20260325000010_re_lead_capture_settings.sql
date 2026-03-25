INSERT INTO site_settings (key, value, description)
VALUES 
    ('re_lc_title', 'Let''s Connect', 'Title for Real Estate form'),
    ('re_lc_desc', 'Join the deal flow or request financing. Select your areas of interest below.', 'Description for Real Estate form'),
    ('re_lc_label', 'Registry Email Address', 'Email label on RE form'),
    ('re_lc_placeholder', 'investor@domain.com', 'Email placeholder on RE form'),
    ('re_lc_btn', 'SUBMIT INQUIRY', 'Submit button on RE form'),
    ('re_options_json', '{"buying": "Buying a Home", "selling": "Selling a Home", "loan": "Getting a real estate investment loan", "networking": "Connecting with other investors"}', 'JSON options for RE form')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;
