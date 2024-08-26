// src/config.rs

#[derive(Debug, Clone)]
pub struct Config {
    pub contentful_space_id: String,
    pub contentful_management_token: String,
}

impl Config {
    pub fn load_from_env() -> Config {
        let app_config: Config = Config{
            contentful_space_id: std::env::var("CONTENTFUL_SPACE_ID")
                .expect("CONTENTFUL_SPACE_ID missing from environment"),
            contentful_management_token: std::env::var("CONTENTFUL_MANAGEMENT_TOKEN")
                .expect("CONTENTFUL_MANAGEMENT_TOKEN missing from environment"),
        };
        println!("config loaded from env {:?}", app_config);
        app_config

    }
}
