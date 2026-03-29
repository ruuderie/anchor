use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::nav::Nav;
use crate::components::footer::Footer;
use crate::pages::admin::Admin;
use crate::pages::blog::Blog;
use crate::pages::certifications::Certifications;
use crate::pages::landing::Landing;
use crate::pages::projects::Projects;
use crate::pages::resume::Resume;
use crate::pages::dynamic_landing::DynamicLanding;
use crate::pages::bitcoin::BitcoinDashboard;

#[cfg(feature = "ssr")]
static PAGE_VIEW_CACHE: std::sync::OnceLock<moka::future::Cache<String, bool>> = std::sync::OnceLock::new();

#[cfg(feature = "ssr")]
fn get_view_cache() -> moka::future::Cache<String, bool> {
    PAGE_VIEW_CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .time_to_live(std::time::Duration::from_secs(3600))
            .max_capacity(10_000)
            .build()
    }).clone()
}

#[server(RecordPageView, "/api")]
pub async fn record_page_view(path: String) -> Result<(), ServerFnError> {
    use axum::Extension;
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let headers = extract::<HeaderMap>().await.unwrap_or_default();
    
    let user_agent = headers.get(axum::http::header::USER_AGENT).and_then(|h| h.to_str().ok()).unwrap_or("unknown").to_string();
    let ip = headers.get("x-forwarded-for").and_then(|h| h.to_str().ok()).unwrap_or("unknown").to_string();
    
    let cache_key = format!("{}:{}:{}", ip, user_agent, path);
    let cache = get_view_cache();
    
    if cache.contains_key(&cache_key) {
        return Ok(());
    }
    cache.insert(cache_key, true).await;

    let _ = sqlx::query("INSERT INTO page_views (path, user_agent) VALUES ($1, $2)")
        .bind(path)
        .bind(user_agent)
        .execute(&state.pool)
        .await;
    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ruuderie_ai.css"/>
        <Title text="Ruud Salym Erie - Technical Architect"/>
        <Meta name="description" content="Technical Architect and Software Engineer specializing in Rust, Salesforce, and high-performance enterprise applications."/>
        <Meta property="og:title" content="Ruud Salym Erie - Technical Architect"/>
        <Meta property="og:description" content="Technical Architect and Software Engineer specializing in Rust, Salesforce, and high-performance enterprise applications."/>
        <Meta property="og:type" content="website"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content="Ruud Salym Erie - Technical Architect"/>
        <Meta name="twitter:description" content="Technical Architect and Software Engineer specializing in Rust, Salesforce, and high-performance enterprise applications."/>

        {
            let settings_resource = create_resource(|| (), |_| crate::pages::landing::get_site_settings());
            view! {
                <Suspense fallback=move || view! {}>
                    {move || {
                        let settings = settings_resource.get().unwrap_or(Ok(crate::pages::landing::SiteSettings::default())).unwrap_or_default();
                        let gcode = settings.google_analytics_id;
                        if !gcode.is_empty() {
                            let gurl = format!("https://www.googletagmanager.com/gtag/js?id={}", gcode);
                            let ascript = format!("window.dataLayer = window.dataLayer || []; function gtag(){{dataLayer.push(arguments);}} gtag('js', new Date()); gtag('config', '{}');", gcode);
                            view! {
                                <Script src=gurl />
                                <Script>{ascript}</Script>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                </Suspense>
            }
        }

        <Router>
            <Nav />
            {
                view! { <PageViewTracker /> }
            }
            <Routes>
                <Route path="/" view=Landing/>
                <Route path="/resume" view=Resume/>
                <Route path="/work" view=|| view! { <Redirect path="/resume" /> }/>
                <Route path="/projects" view=Projects/>
                <Route path="/blog" view=Blog/>
                <Route path="/certifications" view=Certifications/>
                <Route path="/investments/real-estate" view=|| view! { <Redirect path="/p/real-estate-ventures" /> }/>
                <Route path="/investments/bitcoin" view=BitcoinDashboard/>
                <Route path="/p/:slug" view=DynamicLanding/>
                <Route path="/admin" view=Admin/>
                <Route path="/*any" view=|| view! { <div class="pt-32 px-[8.5rem]">"Not Found"</div> }/>
            </Routes>
            <Footer />
        </Router>
    }
}

#[component]
pub fn PageViewTracker() -> impl IntoView {
    let location = use_location();
    create_effect(move |_| {
        let path = location.pathname.get();
        spawn_local(async move {
            let _ = record_page_view(path).await;
        });
    });
    view! { <div class="hidden"></div> }
}

pub fn shell(_options: leptos::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" id="leptos" href="/pkg/ruuderie_ai.css"/>
                <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;800&family=JetBrains+Mono:wght@400;500&display=swap" />
                <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&display=swap" />
            </head>
            <body class="text-on-surface selection:bg-secondary-container selection:text-on-secondary-container">
                <App/>
                <script type="module">
                    r#"
                    import init from '/pkg/ruuderie_ai.js';
                    init('/pkg/ruuderie_ai.wasm');
                    "#
                </script>
            </body>
        </html>
    }
}
