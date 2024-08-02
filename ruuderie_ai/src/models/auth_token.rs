#[cfg(feature = "ssr")]
use crate::entities::auth_token::server::{
    Entity as AuthToken, Entity as AuthTokenEntity, Model as AuthTokenModel,
};

use crate::models::user::User;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::prelude::*;
use rand::{distributions::Standard, thread_rng, Rng};
use sea_orm::{prelude::DateTimeUtc, DbErr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AuthToken {
    pub id: Uuid,

    pub user_id: Uuid, // Assuming you have a 'users' table with Uuid primary keys

    pub token: String, // Store the hashed or encrypted token

    pub expires_at: DateTimeUtc,

    pub purpose: Option<String>,

    pub created_at: DateTimeUtc,

    pub used_at: Option<DateTimeUtc>,
}
// Assuming you have a User entity and its ActiveModel
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

async fn create_auth_token(
    db: &DatabaseConnection,
    user: &User, // Fetch the user based on their ID
    purpose: Option<String>,
) -> Result<AuthTokenModel, DbErr> {
    let token = generate_random_token(); // Implement your token generation logic
    let hashed_token = hash_token(&token); // Implement your hashing logic

    let expiration = chrono::Utc::now() + chrono::Duration::minutes(30);

    let token_model = AuthTokenActiveModel {
        user_id: Set(user.id),
        token: Set(hashed_token),
        expires_at: Set(expiration.into()),
        purpose: Set(purpose),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    token_model.insert(db).await
}
// generate_random_token
pub fn generate_random_token() -> String {
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}

// hash_token
pub fn hash_token(token: &str) -> String {
    hash(token, DEFAULT_COST).unwrap()
}
// verify_token
pub fn verify_token(token: &str, hashed_token: &str) -> bool {
    verify(token, hashed_token).unwrap()
}
