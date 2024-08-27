use crate::auth_token::{ActiveModel as AuthTokenActiveModel, Model as AuthTokenModel};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, DbErr};
use crate::models::user::User;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

async fn create_auth_token(
    db: &DatabaseConnection,
    user: &User,
    purpose: Option<String>,
) -> Result<AuthTokenModel, DbErr> {
    let token = generate_random_token();
    let hashed_token = hash_token(&token);

    let expiration = chrono::Utc::now() + chrono::Duration::minutes(30);

    let mut token_model = AuthTokenActiveModel{
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        token: Set(hashed_token),
        expires_at: Set(expiration),
        purpose: Set(purpose),
        created_at: Set(chrono::Utc::now()),
        used_at: Set(None),
        ..Default::default()
    };

    // Insert the model into the database
    let result = token_model.insert(db).await?;
    Ok(result)
}

pub fn generate_random_token() -> String {
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

pub fn hash_token(token: &str) -> String {
    hash(token, DEFAULT_COST).unwrap()
}

pub fn verify_token(token: &str, hashed_token: &str) -> bool {
    verify(token, hashed_token).unwrap()
}
