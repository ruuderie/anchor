use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(PartialEq, Eq, Debug)]
pub enum LoginAction {
    Granted(UserType),
    Denied,
}
#[derive(PartialEq, Debug, Eq, Clone, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum UserType {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "User")]
    User,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub user_type: UserType,
}
