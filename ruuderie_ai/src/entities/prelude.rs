cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
    pub use super::article::server::Entity as Article;
    pub use super::articlecategory::server::Entity as ArticleCategory;
    pub use super::auth_token::server::Entity as AuthToken;
    pub use super::category::server::Entity as Category;
    pub use super::comment::server::Entity as Comment;
    pub use super::user::server::Entity as User;
    pub use super::landing_page::server::Entity as LandingPage;
}}
