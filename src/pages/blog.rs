use leptos::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PostRecord {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub tags: Vec<String>,
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<PostRecord>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT id, slug, title, content, to_char(created_at, 'YYYY.MM.DD') as created_at, tags FROM blog_posts ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await?;
        
    let posts = rows.into_iter().map(|row| PostRecord {
        id: row.get("id"),
        slug: row.get("slug"),
        title: row.get("title"),
        content: row.get("content"),
        created_at: row.get("created_at"),
        tags: row.get::<Vec<String>, _>("tags"),
    }).collect();
    
    Ok(posts)
}

#[server(AddPost, "/api")]
pub async fn add_post(slug: String, title: String, content: String, tags: Vec<String>) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("INSERT INTO blog_posts (slug, title, content, tags) VALUES ($1, $2, $3, $4)").bind(slug).bind(title).bind(content).bind(tags).execute(&state.pool).await?;
    Ok(())
}

#[server(UpdatePost, "/api")]
pub async fn update_post(id: i32, slug: String, title: String, content: String, tags: Vec<String>) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("UPDATE blog_posts SET slug = $1, title = $2, content = $3, tags = $4 WHERE id = $5").bind(slug).bind(title).bind(content).bind(tags).bind(id).execute(&state.pool).await?;
    Ok(())
}

#[server(DeletePost, "/api")]
pub async fn delete_post(id: i32) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("DELETE FROM blog_posts WHERE id = $1").bind(id).execute(&state.pool).await?;
    Ok(())
}

#[component]
pub fn Blog() -> impl IntoView {
    let posts_resource = create_resource(|| (), |_| async move {
        get_posts().await.unwrap_or_else(|_| vec![])
    });

    view! {
        <main class="pt-32 pb-24 px-6 md:px-[8.5rem] bg-surface-container-low min-h-screen">
            <header class="mb-24 max-w-4xl">
                <div class="inline-block bg-secondary-container/20 px-3 py-1 mb-6 uppercase">
                    <span class="font-label text-[0.6875rem] text-secondary font-bold tracking-tighter">"ENGINEERING DISSERTATIONS"</span>
                </div>
                <h1 class="text-5xl md:text-7xl font-extrabold text-primary tracking-[-0.02em] leading-none mb-8 uppercase">
                    "TECHNICAL WRITING"
                </h1>
                <p class="text-lg text-on-surface-variant max-w-2xl leading-relaxed">
                    "Internal documentation on distributed systems architecture, Bitcoin cryptography, Salesforce APEX algorithms, and low-latency infrastructure design."
                </p>
            </header>

            <Suspense fallback=move || view! { <div class="text-on-surface-variant font-bold jetbrains uppercase">"Fetching remote Markdown streams..."</div> }>
                <div class="space-y-12 max-w-4xl">
                    {move || {
                        let posts = posts_resource.get().unwrap_or_default();
                        posts.into_iter().map(|post| {
                            // Convert markdown preview dynamically using pulldown-cmark
                            let parser = pulldown_cmark::Parser::new(&post.content);
                            let mut html_output = String::new();
                            pulldown_cmark::html::push_html(&mut html_output, parser);

                            view! {
                                <article class="bg-surface-container p-8 hover:bg-surface-container-high transition-colors group cursor-pointer border-l-4 border-transparent hover:border-secondary">
                                    <div class="flex flex-col md:flex-row md:justify-between items-start mb-4 gap-4">
                                        <h3 class="text-2xl font-bold text-primary group-hover:text-secondary transition-colors truncate">{post.title}</h3>
                                        <span class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider whitespace-nowrap pt-1">
                                            {post.created_at} " // " {post.slug}
                                        </span>
                                    </div>
                                    <div class="text-on-surface-variant leading-relaxed text-sm mb-6 max-w-2xl prose prose-invert prose-p:text-sm prose-a:text-secondary prose-a:no-underline hover:prose-a:underline" inner_html=html_output></div>
                                    <div class="flex flex-wrap gap-4">
                                        {post.tags.into_iter().map(|tag| view! {
                                            <span class="bg-surface-container-highest px-3 py-1 jetbrains text-[0.65rem] font-bold text-on-surface-variant uppercase">{tag}</span>
                                        }).collect_view()}
                                    </div>
                                </article>
                            }
                        }).collect_view()
                    }}
                </div>
            </Suspense>
        </main>
    }
}
