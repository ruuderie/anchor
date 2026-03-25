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
use crate::pages::real_estate::RealEstate;

#[server(RecordPageView, "/api")]
pub async fn record_page_view(path: String) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    
    let _ = sqlx::query("INSERT INTO page_views (path) VALUES ($1)")
        .bind(path)
        .execute(&state.pool)
        .await;
    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ruuderie_ai.css"/>
        <Title text="Ruud Salym Erie - Systems Architect"/>

        <Router>
            <Nav />
            {
                view! { <PageViewTracker /> }
            }
            <Routes>
                <Route path="/" view=Landing/>
                <Route path="/resume" view=Resume/>
                <Route path="/projects" view=Projects/>
                <Route path="/blog" view=Blog/>
                <Route path="/certifications" view=Certifications/>
                <Route path="/real-estate" view=RealEstate/>
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
