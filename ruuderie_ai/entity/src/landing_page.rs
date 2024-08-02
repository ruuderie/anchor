#[cfg(feature = "ssr")]
pub mod server {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "landing_pages")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "uuid")]
        pub id: Uuid,
        #[sea_orm(column_type = "Text")]
        pub heading: String,
        #[sea_orm(column_type = "Text")]
        pub subheading: String,
        #[sea_orm(column_type = "Text")]
        pub call_to_action: String,
        #[sea_orm(column_type = "Text")]
        pub video_url: String,
        #[sea_orm(column_type = "Array(Text)")]
        pub company_logos: Vec<String>,
        #[sea_orm(column_type = "Array(Text)")]
        pub benefits: Vec<String>,
        #[sea_orm(column_type = "Array(Text)")]
        pub how_it_works: Vec<String>,
        #[sea_orm(column_type = "Array(Text)")]
        pub testimonials: Vec<String>,
        #[sea_orm(column_type = "Array(Text)")]
        pub faq: Vec<String>,
        #[sea_orm(column_type = "Text")]
        pub footer: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}

    impl Related<super::other::Entity> for Entity {
        fn to() -> RelationDef {
            panic!("No RelationDef")
        }

        fn via() -> Option<RelationDef> {
            None
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
