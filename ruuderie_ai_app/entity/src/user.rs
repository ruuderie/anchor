use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::user::UserType;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub password_hash: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(column_type = "Text")]
    pub user_type: UserType,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Article,
    Comment,
    AuthToken,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Article => Entity::has_many(super::article::Entity).into(),
            Self::Comment => Entity::has_many(super::comment::Entity).into(),
            Self::AuthToken => Entity::has_many(super::auth_token::Entity).into(),
        }
    }
}
impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}
impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}
impl Related<super::auth_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthToken.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
