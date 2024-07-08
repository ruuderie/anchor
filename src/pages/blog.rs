use crate::config::Config;
use crate::error_template::{AppError, ErrorTemplate};
//use contentful::{models::SystemProperties, ContentfulManagementClient, QueryBuilder}; // Import QueryBuilder
use crate::components::post::Post; // Import Post struct
use crate::models::blog::Post;
use crate::services::contentful_services::get_blog_posts; // Import get_blog_posts function
use leptonic::prelude::*;
use leptos::html::P;
use leptos::*;
use serde::Deserialize;
use serde_json::json;
use std::env; // We can still use this for potential expansion

#[component]
pub fn Blog() -> impl IntoView {
    println!("Blog loaded");
    // For fetching data, you'd use your Contentful service here
    // Create a list of sample Post instances
    let sample_posts = vec![
        Post {
            id: "1".to_string(),
            name: "Sample Post 1".to_string(),
            title: "My First Blog Post".to_string(),
            body: "This is the body of my first sample blog post.".to_string(),
            image: Some("https://www.example.com/image1.jpg".to_string()), // Placeholder image URL
            recommended_posts: None,
        },
        Post {
            id: "2".to_string(),
            name: "Sample Post 2".to_string(),
            title: "Another Exciting Post".to_string(),
            body: "This is the content of my second sample blog post.".to_string(),
            image: Some("https://www.example.com/image2.jpg".to_string()),
            recommended_posts: None,
        },
    ];

    view! {

            <div id="blog_posts">
                <h1>Blog Posts</h1>
                <For
                    each=move || { sample_posts.clone() }
                    key=|post| post.id.clone()
                    children=move |post: Post| {
                        view! {

                            <Post data={post} />
                        }
                    }
                />
            </div>


    }
}
