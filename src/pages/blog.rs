use crate::config::Config;
use crate::error_template::{AppError, ErrorTemplate}; // Import if not already used in your project
use contentful::{models::SystemProperties, ContentfulManagementClient};
use leptonic::prelude::*;
use leptos::*;
use serde_json::json;
use std::env;

#[component]
pub fn Blog() -> impl IntoView {
    let config = Config::load_from_env();
    let client = contentful::Client::new(
        config.contentful_management_token,
        config.contentful_space_id,
    );
    let slug = create_signal(String::new()); // For dynamic routing
    let posts = create_resource(slug, move || async move {
        let entries: Vec<BlogPost> = client
            .entries()
            .get()
            .with("content_type", "blogPost")
            .all()
            .await?;
        Ok(entries)
    });

    view! {
      <main>
        <h2>"Latest Blog Posts"</h2>
        {
          match posts.read() {
            Some(data) => view! {
              <ul>
                {data.iter().map(|post| view!{ <li><A href={format!("/blog/{}", post.slug)}>{post.title.clone()}</A></li>})}
              </ul>
            },
            None => view! { <p>"Loading..."</p> }
          }
        }
      </main>
    }
}
