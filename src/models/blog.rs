use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Post {
    pub id: String,
    pub name: String,
    pub title: String,
    pub body: String,
    pub image: Option<String>,
    pub recommended_posts: Option<Vec<String>>,
}
