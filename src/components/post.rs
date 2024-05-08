use crate::config::Config;
use crate::error_template::{AppError, ErrorTemplate}; // Import if not already used in your project
use contentful::{models::SystemProperties, ContentfulManagementClient};
use leptonic::prelude::*;
use leptos::*;
use serde_json::json;
use std::env;

#[component]
pub fn Post() -> impl IntoView {
    let config = Config::load_from_env();

    let slug = create_signal(String::new()); // For dynamic routing
    let post = create_resource(slug, move || async move {
        let client = contentful::Client::new(
            config.contentful_management_token,
            config.contentful_space_id,
        );

        // Assuming posts are published, use Contentful Delivery API (CDA)
        // for production scenarios due to its optimization for read operations

        let entry: BlogPost = client
            .entries()
            .get()
            .with("content_type", "blogPost")
            .with("fields.slug", &slug())
            .one()
            .await?;
        println!("{:?}", entry);
        Ok(entry)
    });

    view! {
        <main>
            {
                match post.read() {
                    Some(data) => view! { cx,
                        <h1>{data.title.clone()}</h1>
                        <div dangerously_set_inner_html={data.body.clone()} />
                    },
                    None => view! { cx, <p>"Loading..."</p> }
                }
            }

        </main>
    }
}
