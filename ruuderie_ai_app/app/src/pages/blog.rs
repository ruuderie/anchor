use crate::error_template::{AppError, ErrorTemplate};
//use contentful::{models::SystemProperties, ContentfulManagementClient, QueryBuilder}; // Import QueryBuilder
use crate::components::post::Post; // Import Post struct
use entity::article::Model as Article;
//use crate::services::contentful_services::get_blog_posts; // Import get_blog_posts function

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
    let sample_posts: Vec<Article> = vec![];

    view! {

            <div id="blog_posts">
                <h1>Blog Posts</h1>
                <For
                    each=move || { sample_posts.clone() }
                    key=|post| post.id.clone()
                    children=move |post: Article| {
                        view! {

                            <Post data={post} />
                        }
                    }
                />
            </div>


    }
}
