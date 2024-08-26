use crate::utils::config::Config;
use std::env;

pub async fn get_blog_posts(config: &Config)  -> Result<(), Box<dyn std::error::Error>>{
    let space_id = env::var("CONTENTFUL_SPACE_ID")
        .map_err(|_| "CONTENTFUL_SPACE_ID is not set in the environment")?;
    
    // Use the space_id variable here
    println!("Contentful Space ID: {}", space_id);

    // Rest of your function...
    Ok(())
   /* let client = ContentfulManagementClient::new(
        &config.contentful_management_token,
        &config.contentful_space_id,
    );
    let insights = client.get_entry("ZkzzTdQNVJIryHl5uxu9w").await?.unwrap();
    println!("Contentful Entry: {:?}", insights); */

}

