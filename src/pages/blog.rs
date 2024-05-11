use crate::config::Config;
use crate::error_template::{AppError, ErrorTemplate};
use contentful::{models::SystemProperties, ContentfulManagementClient, QueryBuilder}; // Import QueryBuilder
use leptonic::prelude::*;
use leptos::*;
use serde_json::json;
use std::env;
use crate::components::Post
use serde::Deserialize; // We can still use this for potential expansion

#[component]
pub fn Blog() -> impl IntoView {
    let config = Config::load_from_env(); // Assuming you have config

    // For fetching data, you'd use your Contentful service here

    let posts = create_resource((), move || async move {
        // Using () for the trigger
        // Placeholder: Sample list of posts
        let sample_posts = vec![
            Post {
                title: "My First Blog Post".to_string(),
                slug: "first-post".to_string(), /*...*/
            },
            Post {
                title: "Another Post".to_string(),
                slug: "another-post".to_string(), /*...*/
            },
        ];

        Ok(Some(sample_posts)) // Return sample posts as Some(data)
    });

    view! {
        <main>
            <h2>"Latest Blog Posts"</h2>
            {
                match posts.read() {
                    Some(Some(data)) => view! {
                        <ul>
                            {data.iter().map(|post| view!{
                                <li><A href={format!("/blog/{}", post.slug)}>{post.title.clone()}</A></li>
                            })}
                        </ul>
                    },
                    Some(None) => view! { <p>"Loading..."</p> },
                    None => view! { <ErrorTemplate/> }
                }
            }
        </main>
    }
}
