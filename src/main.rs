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
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://ruud_admin:R3sUm3_S3cUr3@localhost:5432/ruuderie_ai".into()
    });
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        pool,
    };

    let site_root = leptos_options.site_root.clone();

    let app = Router::new()
        .route(
            "/api/*fn_name",
            axum::routing::get(leptos_axum::handle_server_fns).post(leptos_axum::handle_server_fns),
        )
        .route(
            "/robots.txt",
            axum::routing::get({
                let site_root = site_root.clone();
                move || {
                    let path = format!("{}/robots.txt", site_root);
                    async move { std::fs::read_to_string(path).unwrap_or_default() }
                }
            }),
        )
        .route(
            "/sitemap.xml",
            axum::routing::get({
                let site_root = site_root.clone();
                move || {
                    let path = format!("{}/sitemap.xml", site_root);
                    async move {
                        (
                            [(axum::http::header::CONTENT_TYPE, "application/xml")],
                            std::fs::read_to_string(path).unwrap_or_default(),
                        )
                    }
                }
            }),
        )
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", site_root)))
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || leptos::provide_context(app_state.clone())
            },
            { move || view! { <App/> } },
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
