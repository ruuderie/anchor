use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "articles")]
pub struct Article {
    #[sea_orm(primary_key, column_type = "uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    #[sea_orm(column_type = "uuid")]
    pub author_id: Uuid,
    #[sea_orm(column_type = "uuid")]
    pub category_id: Uuid,
    #[sea_orm(column_type = "timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum ArticleRelation {
    User,
    Category,
}

impl Related<super::user::User> for Article {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::category::Category> for Article {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for Article {}
