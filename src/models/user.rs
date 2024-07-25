use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(PartialEq, Eq, Debug)]
pub enum LoginAction {
    Granted(UserType),
    Denied,
}
pub enum UserType {
    Admin,
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
