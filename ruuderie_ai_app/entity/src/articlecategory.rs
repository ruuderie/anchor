
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub category_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub article_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Article,
    Category,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Article => Entity::has_one(super::article::Entity).into(),
            Self::Category => Entity::has_one(super::category::Entity).into(),
        }
    }
}
impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}
impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
