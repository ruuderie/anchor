use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "comments")]
pub struct Comment {
    #[sea_orm(primary_key, column_type = "uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "uuid")]
    pub article_id: Uuid,
    #[sea_orm(column_type = "uuid")]
    pub user_id: Uuid,
    #[sea_orm(column_type = "text")]
    pub content: String,
    #[sea_orm(column_type = "timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum CommentRelation {
    Article,
    User,
}

impl Related<super::article::Article> for Comment {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}

impl Related<super::user::User> for Comment {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for Comment {}
