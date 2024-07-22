use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
