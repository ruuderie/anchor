#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use ruuderie_ai::app::*;
    use ruuderie_ai::state::AppState;
    use sqlx::PgPool;
    use tower_http::services::ServeDir;

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Initialize Database
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ruud_admin:R3sUm3_S3cUr3@localhost:5432/ruuderie_ai".into());
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        pool,
    };

    let site_root = &leptos_options.site_root;

    let app = Router::new()
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", site_root)))
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || leptos::provide_context(app_state.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .layer(axum::Extension(app_state.clone()))
        .with_state(app_state);

    leptos::logging::log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {}
