use crate::config::Config; // If you still have the Config struct
use crate::error_template::{AppError, ErrorTemplate}; // Assuming you keep this
use leptonic::prelude::*;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize, PartialEq, Clone, Debug)]
pub struct Post {
    pub id: String,
    pub name: String,
    pub title: String,
    pub body: String,
    pub image: Option<String>,
    pub recommended_posts: Option<Vec<String>>,
}

#[component]
pub fn Post( data: Post) -> impl IntoView {

    view! { 
        <main> 
        
                <h1>{data.title.clone()}</h1> 
                <img src={data.image.clone().unwrap_or_default()} alt={data.title} />
                <div>{data.body}</div>

            
            </main>
        } 
    }
