use crate::services::database as db;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "auth_tokens")] // Replace with your actual table name
pub struct Model {
    #[sea_orm(primary_key, auto_generate = true, column_type = "Uuid")]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid")]
    pub user_id: Uuid, // Assuming you have a 'users' table with Uuid primary keys

    #[sea_orm(unique)]
    pub token: String, // Store the hashed or encrypted token
    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub expires_at: DateTimeUtc,

    #[sea_orm(column_type = "String(Some(50))")]
    pub purpose: Option<String>,

    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: DateTimeUtc,

    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub used_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::has_one(super::user::Entity).into(),
        }
    }
}
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::auth_token::ActiveModel {
    //pass in user entity to get user_id
    fn new() -> Self {
        let expiration = chrono::Utc::now() + chrono::Duration::minutes(30);

        Self {
            id: Set(Uuid::new_v4()),
            expires_at: Set(expiration),
            ..ActiveModelTrait::default()
        }
    }

    /// Will be triggered before insert / update
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }

    /// Will be triggered after insert / update
    async fn after_save<C>(model: Model, db: &C, insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(model)
    }

    /// Will be triggered before delete
    async fn before_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }

    /// Will be triggered after delete
    async fn after_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}
