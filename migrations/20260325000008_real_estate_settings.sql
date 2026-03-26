INSERT INTO site_settings (key, value)
VALUES 
    ('real_estate_title', 'Real Estate Ventures.'),
    ('real_estate_desc', 'I am an active real estate investor and landlord always looking for the next deal or strategic partnership. Beyond acquiring properties, I leverage my network as a loan broker to structure investment capital.')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;
