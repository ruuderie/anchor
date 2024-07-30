use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

pub async fn establish_connection() -> Result<DatabaseConnection, sea_orm::DbErr> {
    dotenv().ok();

    let host = env::var("POSTGRES_ENDPOINT").expect("DB_HOST must be set");
    let port = env::var("POSTGRES_PORT").expect("DB_PORT must be set");
    let user = env::var("POSTGRES_USER").expect("DB_USER must be set");
    let password = env::var("POSTGRES_PASSWORD").expect("DB_PASSWORD must be set");
    let dbname = env::var("POSTGRES_DB").expect("DB_NAME must be set");

    // Construct the SeaORM connection string
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, dbname
    );

    // Configure the SeaORM connection options
    let mut opt = ConnectOptions::new(&db_url);
    opt.max_connections(16)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(12))
        .sqlx_logging(true)
        .set_schema_search_path("public"); // Optional: Setting default PostgreSQL schema

    // Establish the connection
    Database::connect(opt).await
}
