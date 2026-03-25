use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectRecord {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub impact: String,
    pub tags: Vec<String>,
    pub bullets: Vec<String>,
}

#[server(GetProjects, "/api")]
pub async fn get_projects() -> Result<Vec<ProjectRecord>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT id, title, slug, impact, tags, bullets FROM projects ORDER BY id ASC")
        .fetch_all(&state.pool)
        .await?;
        
    let projs = rows.into_iter().map(|row| ProjectRecord {
        id: row.get("id"),
        title: row.get("title"),
        slug: row.get("slug"),
        impact: row.get("impact"),
        tags: row.get::<Vec<String>, _>("tags"),
        bullets: row.get::<Vec<String>, _>("bullets"),
    }).collect();
    
    Ok(projs)
}

#[component]
pub fn Projects() -> impl IntoView {
    let projs_resource = create_resource(|| (), |_| async move {
        get_projects().await.unwrap_or_else(|_| vec![])
    });

    view! {
        <main class="pt-32 pb-24 px-6 md:px-[8.5rem] bg-surface min-h-screen">
            <header class="mb-24 flex flex-col items-start max-w-4xl border-b-2 border-outline-variant/30 pb-8">
                <div class="inline-block bg-primary-container/20 px-3 py-1 mb-6">
                    <span class="font-label text-[0.6875rem] text-primary font-bold tracking-tighter">"INDEX_REF_05 // ACTIVE REPOS"</span>
                </div>
                <h1 class="text-5xl md:text-7xl font-extrabold text-on-surface tracking-[-0.02em] leading-none mb-4">
                    "SYSTEMS BUILD LOG"
                </h1>
                <p class="text-xl text-on-surface-variant font-medium mt-4 max-w-2xl">
                    "Engineering resilient infrastructures across blockchains, decentralized cloud, and sub-millisecond Rust backends."
                </p>
            </header>

            <Suspense fallback=move || view! { <div class="text-on-surface-variant font-bold jetbrains uppercase">"Indexing project graphs..."</div> }>
                <div class="space-y-16 max-w-5xl">
                    {move || {
                        let projects = projs_resource.get().unwrap_or_default();
                        projects.into_iter().map(|proj| {
                            let repo_link = format!("https://github.com/ruuderie/{}", proj.slug);
                            view! {
                                <article class="bg-surface-container-low p-8 md:p-12 relative border-l-4 border-secondary shadow-none ring-0">
                                    <div class="absolute top-0 right-0 bg-secondary px-3 py-1 text-xs font-bold text-on-primary jetbrains uppercase tracking-widest hidden md:block">
                                        "DEPLOY_ACTIVE"
                                    </div>
                                    <h2 class="text-3xl font-extrabold text-primary mb-2">{proj.title}</h2>
                                    <a href=repo_link.clone() target="_blank" rel="noopener noreferrer" class="text-sm font-label text-outline hover:text-secondary hover:underline transition-colors flex items-center gap-2 mb-6 w-fit cursor-pointer">
                                        <span class="material-symbols-outlined text-sm">"link"</span>
                                        {repo_link}
                                    </a>
                                    <p class="text-lg font-bold text-on-surface mb-6">{proj.impact}</p>
                                    
                                    <div class="flex flex-wrap gap-2 mb-8">
                                        {proj.tags.into_iter().map(|tag| view! {
                                            <div class="bg-surface-container-highest px-3 py-1 text-xs font-bold text-on-surface jetbrains uppercase border-b border-r border-outline-variant/50">
                                                {tag}
                                            </div>
                                        }).collect_view()}
                                    </div>

                                    <ul class="text-on-surface-variant leading-relaxed text-sm space-y-4 list-none p-0 m-0">
                                        {proj.bullets.into_iter().map(|b| view! {
                                            <li class="relative pl-5 before:content-['//'] before:absolute before:-left-1 before:text-secondary before:font-bold before:jetbrains">
                                                {b}
                                            </li>
                                        }).collect_view()}
                                    </ul>
                                </article>
                            }
                        }).collect_view()
                    }}
                </div>
            </Suspense>
        </main>
    }
}
