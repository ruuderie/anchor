use crate::config::Config;
use contentful::{ ContentfulManagementClient};

pub async fn get_blog_posts(config: &Config)  -> Result<(), Box<dyn std::error::Error>>{
   /* let client = ContentfulManagementClient::new(
        &config.contentful_management_token,
        &config.contentful_space_id,
    );
    let insights = client.get_entry("ZkzzTdQNVJIryHl5uxu9w").await?.unwrap();
    println!("Contentful Entry: {:?}", insights); */
    Ok(())

}

