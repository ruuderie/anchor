use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LandingPage {
    pub id: i32,
    pub heading: String,
    pub subheading: String,
    pub call_to_action: String,
    pub video_url: String,
    pub company_logos: Vec<String>,
    pub benefits: Vec<String>,
    pub how_it_works: Vec<String>,
    pub testimonials: Vec<String>,
    pub faq: Vec<String>,
    pub footer: String,
}
