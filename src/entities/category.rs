use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Category {
    #[sea_orm(primary_key, column_type = "uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "text")]
    pub name: String,
    #[sea_orm(column_type = "timestamp")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "timestamp")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum CategoryRelation {}

impl Related<super::other::Entity> for Category {
    fn to() -> RelationDef {
        panic!("No RelationDef")
    }

    fn via() -> Option<RelationDef> {
        None
    }
}

impl ActiveModelBehavior for Category {}
