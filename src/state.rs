use axum::extract::FromRef;
use leptos::LeptosOptions;
use sqlx::PgPool;
use std::sync::Arc;
#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
}
