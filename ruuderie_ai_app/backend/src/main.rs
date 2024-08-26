mod services;
mod utils;

use axum_macros::FromRef;
use dotenv::dotenv;
use std::env;
use entity::db::{DBConfig, DB};
use http::header;
use app::app::*;
use utils::config::Config;
use crate::services::fileserv::file_and_error_handler;
use crate::services::contentful_services::get_blog_posts;
use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: DB,
    pub config: Config,
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    req: http::Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.db.clone());
            provide_context(app_state.config.clone());
        },
        req,
    )
    .await
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: http::Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(app_state.db.clone());
            provide_context(app_state.config.clone());
        },
        || view! { <App/> },
    );
    handler(req).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    // Ensure LEPTOS_OUTPUT_NAME is set
    if env::var("LEPTOS_OUTPUT_NAME").is_err() {
        env::set_var("LEPTOS_OUTPUT_NAME", "ruuderie-ai");
    }

    // Setup logging
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(false)
        .with_thread_ids(false);
    
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer)
        .init();

    // Load configuration
    let config = Config::load_from_env();
    if let Err(e) = get_blog_posts(&config).await {
        eprintln!("Error getting blog posts: {}", e);
        // Decide if you want to continue or exit here
    }

    let conf = match get_configuration(Some("Cargo.toml")).await {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Failed to load Leptos configuration: {:?}", e);
            return Err(e.into());
        }
    };
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    // Setup database
    let db_conf = DBConfig::figment().extract::<DBConfig>()?;
    let db = DB::connect(&db_conf).await?;
    db.run_migrations().await?;

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        db: db.clone(),
        config,
    };

    let routes = generate_route_list(|| view! { <App/> });

    // Build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(SetSensitiveRequestHeadersLayer::new(vec![
            header::AUTHORIZATION,
            header::COOKIE,
        ]))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    tracing::info!("listening on http://{}", &addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}