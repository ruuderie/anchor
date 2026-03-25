INSERT INTO site_settings (key, value) VALUES 
    ('hero_quote', 'Vires in Numeris. Systems architecture is not defined by lines, but by cryptographic proofs and immutable data flows.'),
    ('hero_subtitle', 'SALESFORCE TECHNICAL ARCHITECT // SPECIALIZING IN ENTERPRISE CLOUD SOLUTIONS, LWC, APEX, AND RUST EXTERNAL MICROSERVICES.')
ON CONFLICT DO NOTHING;
