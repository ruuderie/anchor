use crate::config::Config; // If you still have the Config struct
use crate::error_template::{AppError, ErrorTemplate}; // Assuming you keep this
use leptonic::prelude::*;
use leptos::*;
use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
struct Post {
    name: String,
    title: String,
    body: String,
    image: Option<String>,
    recommended_posts: Option<Vec<String>>,
}

#[component]
pub fn Post() -> impl IntoView {
    // Create a signal to hold your post data
    let post = create_signal(create_memo(|_| {
        // Pass a closure here
        // Sample Post Data
        Post {
            name: "Sample Post".to_string(),
            title: "My First Blog Post".to_string(),
            body: "This is the body of my sample blog post.".to_string(),
            image: Some("https://www.example.com/image.jpg".to_string()),
            recommended_posts: None,
        }
    }));
    view! { cx,
        <main> {
            move || {
                let data = post;
                        <h1>{data.title.clone()}</h1>
                        <img src=data.image.clone().unwrap_or_default() alt=data.title.clone() />
                        <div>{data.body.clone()}</div>

            }
        } </main>
    }
}
