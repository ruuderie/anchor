#[cfg(feature = "ssr")]
pub mod server {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "articles")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "Uuid")]
        pub id: Uuid,
        #[sea_orm(column_type = "Text")]
        pub title: String,
        #[sea_orm(column_type = "Text")]
        pub content: String,
        #[sea_orm(column_type = "Uuid")]
        pub author_id: Uuid,
        #[sea_orm(column_type = "Uuid")]
        pub category_id: Uuid,
        pub created_at: DateTime,
        pub updated_at: DateTime,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        User,
        ArticleCategory,
        Comment,
    }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::User => Entity::has_one(super::user::server::Entity).into(),
                Self::ArticleCategory => {
                    Entity::has_many(super::articlecategory::server::Entity).into()
                }
                Self::Comment => Entity::has_many(super::comment::server::Entity).into(),
            }
        }
    }

    impl Related<super::user::server::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }
    impl Related<super::comment::server::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Comment.def()
        }
    }

    impl Related<super::articlecategory::server::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ArticleCategory.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
