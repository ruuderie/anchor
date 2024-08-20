mod services;
mod utils;

use axum_macros::FromRef;
use dotenv::dotenv;
use entity::db::{DBConfig, DB};
use http::{header, HeaderMap, Request};
use app::app::*;
use utils::config::Config;
use crate::{ services::fileserv::file_and_error_handler, services::contentful_services::get_blog_posts};
use axum::{
    body::Body as AxumBody,
    extract::{FromRef, Path, RawQuery, State},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use leptos::*;
use leptos_axum::{handle_server_fns_with_context, LeptosRoutes};
use tower_http::trace::TraceLayer;
use tracing::Level;
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: DB,
    pub config: Config,
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    tracing::info!("serverfn: {:?}", path);
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.db.clone());
            provide_context(app_state.config.clone());
        },
        request,
    )
    .await
}
async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
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

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenv().ok();

    // Setup logging
    let log_filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("tokio", tracing::Level::WARN)
        .with_target("runtime", tracing::Level::WARN);
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(false)
        .with_thread_ids(false);
    let fmt_layer_filtered = fmt_layer.with_filter(log_filter);
    tracing_subscriber::Registry::default()
        .with(fmt_layer_filtered)
        .init();

    // Load configuration
    let config = Config::load_from_env();
    let _blog_posts = get_blog_posts(&config).await;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // Setup database
    let db_conf = DBConfig::figment().extract::<DBConfig>().unwrap();
    let db = DB::connect(&db_conf).await.unwrap();
    db.run_migrations().await.unwrap();

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        db: db.clone(),
        config,
    };

    let routes = generate_route_list(|| view! { <App/> });

    // build our application with a route
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

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
