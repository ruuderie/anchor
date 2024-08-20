use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
