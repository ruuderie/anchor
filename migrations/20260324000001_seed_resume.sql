-- modify jobs schema
ALTER TABLE jobs DROP COLUMN description;
ALTER TABLE jobs ADD COLUMN bullets TEXT[] NOT NULL DEFAULT '{}';
ALTER TABLE jobs ADD COLUMN is_client_project BOOLEAN NOT NULL DEFAULT true;

-- modify projects schema
ALTER TABLE projects DROP COLUMN description;
ALTER TABLE projects ADD COLUMN bullets TEXT[] NOT NULL DEFAULT '{}';

-- modify certifications schema
ALTER TABLE certifications DROP COLUMN description;
ALTER TABLE certifications DROP COLUMN role;
ALTER TABLE certifications DROP COLUMN company;
ALTER TABLE certifications ADD COLUMN title TEXT NOT NULL DEFAULT '';
ALTER TABLE certifications ADD COLUMN is_training BOOLEAN NOT NULL DEFAULT false;

-- Insert Client Projects
INSERT INTO jobs (date_range, role, company, bullets, is_client_project) VALUES
('August 2024 – Present', 'Salesforce Architect/Technical Lead', 'Enterprise Cloud Solutions Inc.', ARRAY[
    'Led two major ERP implementation projects, a commercial heating solution with Field Service Lightning integration and a Salesforce ERP integration with MailChimp, DocuSign, and a custom forecasting model.',
    'Managed all aspects of the projects, including developer oversight, system design, client relations, and overall project management, ensuring timely delivery and client satisfaction.',
    'Architected and oversaw complex data migration strategies for the commercial heating project, seamlessly transferring legacy data into the new Salesforce-based ERP system.',
    'Designed and implemented a custom integration framework to connect Salesforce with external systems (MailChimp, DocuSign) and a bespoke forecasting model, significantly enhancing the client''s operational efficiency and decision-making capabilities.'
], true),
('April 2023 – August 2024', 'Salesforce Architect/Engineer', 'Swan Bitcoin', ARRAY[
    'Redesigned the application architecture to bolster security measures and enhance data accuracy during data exchange.',
    'Managed, wrote, and maintained custom API integrations with external systems, leveraging Apex and Rust programming language for integration testing.',
    'Implemented Salesforce Shield to enhance data security and compliance with industry standards.',
    'Introduced version control system to manage source code efficiently, enabling easy collaboration and tracking changes.',
    'Established a streamlined release process for production, reducing deployment errors and enhancing delivery efficiency.'
], true),
('January 2023 – July 2023', 'Blockchain Developer', 'Zing.It', ARRAY[
    'Developed TypeScript and React unit tests to ensure the reliability and stability of UI components within the blockchain web application.',
    'Implemented an integration with SendGrid to facilitate the automatic sending of emails for key platform events, enhancing user engagement and communication effectiveness.',
    'Employed solidity smart contracts to enable secure and transparent transactional processes within the platform.',
    'Actively participated in code reviews, providing valuable feedback to peers and promoting code quality and best practices.'
], true),
('April 2022 - June 2022', 'Salesforce Consultant', 'Grayscale Investments', ARRAY[
    'Audited Grayscale''s Salesforce systems and codebase to identify areas for improvement and ensure compliance with best practices.',
    'Collaborated with the wealth management and advisory team to understand their sales process and develop solutions to enhance efficiency.',
    'Implemented lead integration from external systems to streamline data capture and management.',
    'Consolidated Grayscale''s data structure to facilitate seamless management of clients at different stages of the sales process.',
    'Provided training and guidance to Grayscale''s team on utilizing Salesforce effectively for their sales operations.'
], true),
('April 2022 – December 2022', 'Salesforce Architect and Senior Developer', 'Principle Studios', ARRAY[
    'Worked with client to build a Transportation Management System using Salesforce from scratch.',
    'Developed a Trigger Framework to support end to end Load management.',
    'Developed a security model that required information to be hidden from users working in different offices, managing, and enabling customer credit and the secure tracking and billing of shipper loads.',
    'Developed capabilities that support the Automatic and Manual Credit Approval, directly reducing the time it takes for agents to complete their work.',
    'Used SFDX for metadata deployment from environment to environment.'
], true),
('April 2020 – April 2022', 'Salesforce Lead Engineer / Implementation Architect', 'Mercury Healthcare', ARRAY[
    'Worked with Key Clients including Prisma Healthcare, Ascension Health and Advocate Aurora Health to implement multiple ISV products across the Sales Cloud, Service Cloud and Marketing Cloud.',
    'Implemented SSO for 50+ projects with Mercury Health Customers connecting Microsoft, Okta, Salesforce, and Appian Cloud.',
    'Developed Continuous Integration Process using SFDX for feature delivery.',
    'Configured Tableau and Snowflake for data visualizations used in Salesforce.'
], true);

-- Insert Employment
INSERT INTO jobs (date_range, role, company, bullets, is_client_project) VALUES
('January 2012 - Present', 'Salesforce Technical Architect / Lead Software Engineer', 'Oplyst International, LLC', ARRAY[
    'Provide ongoing support & technical consultation for medium to large enterprises & non-profits',
    'Einstein Analytics and Discovery implementations with large datasets.',
    'Custom lightning development on Sales Cloud, Service Cloud, Health Cloud, and Nonprofit Starter Pack.',
    'Integrated salesforce instances with external services using REST, Platform Events & Heroku.',
    'Building Product APIs using Rust, Python and Deno Backends'
], false),
('June 2018 - February 2019', 'CRM Technical Manager / Salesforce Architect', 'ZIPARI Inc', ARRAY[
    'Managing a 5 person team of Senior Engineers, in addition to being the Principal Salesforce Architect across all Salesforce Products',
    'Providing technical design, architecture & Api specs for new features',
    'Implemented numerous CI builds for deployment and product packaging using salesforce dx',
    'Successfully designed and built a call center application on Lightning Service Cloud',
    'Completed an internal audit and created a strategy to bring code coverage from 0% to 85% across all products in less than 2 months.'
], false),
('March 2017 - March 2018', 'Salesforce Architect / Lead Software Engineer', 'Evariant Inc', ARRAY[
    'Migrated 30 + companies to lightning to use the latest version of our application.',
    'Re-built Healthcare marketing lead list builder in using lightning components. List builder is used by 50 + health networks across the United States on a day to day basis.',
    'Built reusable lightning components used across Salesforce Classic and Lightning.',
    'Wrote custom REST Api to ingest healthcare case data.'
], false);

-- Insert Projects
INSERT INTO projects (slug, title, impact, tags, bullets) VALUES
('rs-business-directory', 'RS Business Directory', 'Full-Stack Web Application for Business Listings Management', ARRAY['Rust', 'Svelte', 'Axum', 'PostgreSQL', 'Docker'], ARRAY[
    'Architected a high-performance business directory platform using Rust and Svelte, enabling efficient search and management of business listings for thousands of users.',
    'Developed RESTful APIs with Rust and the Axum framework, optimizing data handling and ensuring robust security for user and listing data, achieving sub-100ms response times under load.',
    'Designed a responsive admin interface with Svelte, Vite, and Tailwind CSS, integrating shadcn-svelte components to deliver an intuitive user experience.',
    'Containerized backend and frontend services using Docker, streamlining deployments across cloud environments and ensuring consistency with zero-downtime updates.',
    'Implemented a PostgreSQL-backed data layer with sqlx for scalable storage and retrieval, supporting rapid queries on large datasets and enabling future growth.',
    'Leveraged modern DevOps practices, including CI/CD pipelines with GitHub Actions and pnpm for frontend dependency management, reducing deployment cycles by 40%.'
]),
('basecamp', 'Basecamp', 'Smart contract deployment on the Starknet blockchain', ARRAY['Starknet', 'Cairo', 'Next.js', 'Web3.js', 'snfoundry'], ARRAY[
    'Built a modular Next.js frontend with Web3.js integration, enabling seamless interaction with StarkNet smart contracts, improving user accessibility across diverse devices.',
    'Authored smart contracts and deployment scripts using snfoundry, optimizing gas efficiency on StarkNet and ensuring compliance with Ethereum security standards.',
    'Structured the project for scalability, separating frontend (/packages/nextjs) and blockchain components (/packages/snfoundry), facilitating maintenance and future feature additions.',
    'Integrated Yarn for dependency management and Vitest for unit testing, achieving 90%+ code coverage and ensuring reliability in production environments.'
]),
('loan-landscape', 'Loan Landscape', 'Full-Stack Commercial Loan and Property Analysis Platform', ARRAY['Rust', 'Nuxt.js', 'Vue.js', 'D3.js', 'TypeScript'], ARRAY[
    'Developed a scalable property analysis platform using Nuxt.js, TypeScript, and Rust, delivering real-time insights for property investment decisions.',
    'Implemented a high-throughput data processing pipeline in Rust, handling complex property calculations with sub-second latency, tested up to 10,000 daily queries.',
    'Created interactive data visualization components with Nuxt.js and D3.js, enabling users to explore property metrics intuitively.',
    'Designed a responsive frontend with Vue.js-based Nuxt.js, ensuring cross-platform compatibility and accessibility, validated through user testing.',
    'Architected a modular system separating frontend, backend, and data processing layers, enhancing maintainability and enabling independent scaling of services.'
]);

-- Insert Certifications
INSERT INTO certifications (date_range, title, is_training) VALUES
('', 'Salesforce Certified Administrator', false),
('', 'Salesforce Certified Platform Developer I', false),
('', 'Salesforce Certified Sharing and Visibility Architect', false),
('', 'Salesforce Certified Agentforce Specialist', false),
('', 'Salesforce Certified Service Cloud Consultant', false),
('', 'Salesforce Certified AI Associate', false),
('', 'Salesforce Certified Platform App Builder', false),
('', 'Einstein Discover Training at Salesforce Office in Herndon, VA', true),
('', 'Einstein Analytics Advanced Training at Salesforce Office in Herndon, VA', true);
