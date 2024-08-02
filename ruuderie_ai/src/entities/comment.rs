#[cfg(feature = "ssr")]
pub mod server {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "comments")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "Uuid")]
        pub id: Uuid,
        #[sea_orm(column_type = "Uuid")]
        pub article_id: Uuid,
        #[sea_orm(column_type = "Uuid")]
        pub user_id: Uuid,
        pub content: String,
        pub created_at: DateTime,
        pub updated_at: DateTime,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Article,
        User,
    }
    impl Related<super::article::server::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Article.def()
        }
    }

    impl Related<super::user::server::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::User => Entity::has_one(super::user::server::Entity).into(),
                Self::Article => Entity::has_one(super::article::server::Entity).into(),
            }
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
