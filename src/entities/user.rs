use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct User {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum UserRelation {}

impl Related<super::other::Entity> for User {
    fn to() -> RelationDef {
        panic!("No RelationDef")
    }

    fn via() -> Option<RelationDef> {
        None
    }
}

impl ActiveModelBehavior for User {}
