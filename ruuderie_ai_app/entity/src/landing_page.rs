use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "landing_pages")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
