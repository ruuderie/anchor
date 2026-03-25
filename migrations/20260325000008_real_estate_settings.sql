INSERT INTO site_settings (key, value, description)
VALUES 
    ('real_estate_title', 'Real Estate Ventures.', 'Title on real estate page'),
    ('real_estate_desc', 'I am an active real estate investor and landlord always looking for the next deal or strategic partnership. Beyond acquiring properties, I leverage my network as a loan broker to structure investment capital.', 'Description on real estate page')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;
