use leptos::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct FooterItemRecord {
    pub id: i32,
    pub label: String,
    pub href: Option<String>,
    pub display_order: i32,
    pub is_visible: bool,
}

#[server(GetFooterItems, "/api")]
pub async fn get_footer_items() -> Result<Vec<FooterItemRecord>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT * FROM footer_items WHERE is_visible = true ORDER BY display_order ASC")
        .fetch_all(&state.pool).await?;
    let mut items = Vec::new();
    for row in rows {
        items.push(FooterItemRecord {
            id: row.get("id"),
            label: row.get("label"),
            href: row.get("href"),
            display_order: row.get("display_order"),
            is_visible: row.get("is_visible"),
        });
    }
    Ok(items)
}

#[server(GetAllFooterItems, "/api")]
pub async fn get_all_footer_items() -> Result<Vec<FooterItemRecord>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    use crate::auth::check_session;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT * FROM footer_items ORDER BY display_order ASC")
        .fetch_all(&state.pool).await?;
    let mut items = Vec::new();
    for row in rows {
        items.push(FooterItemRecord {
            id: row.get("id"),
            label: row.get("label"),
            href: row.get("href"),
            display_order: row.get("display_order"),
            is_visible: row.get("is_visible"),
        });
    }
    Ok(items)
}

#[server(AddFooterItem, "/api")]
pub async fn add_footer_item(label: String, href: Option<String>, display_order: i32, is_visible: bool) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("INSERT INTO footer_items (label, href, display_order, is_visible) VALUES ($1, $2, $3, $4)")
        .bind(label).bind(href).bind(display_order).bind(is_visible)
        .execute(&state.pool).await?;
    Ok(())
}

#[server(UpdateFooterItem, "/api")]
pub async fn update_footer_item(id: i32, label: String, href: Option<String>, display_order: i32, is_visible: bool) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("UPDATE footer_items SET label = $1, href = $2, display_order = $3, is_visible = $4 WHERE id = $5")
        .bind(label).bind(href).bind(display_order).bind(is_visible).bind(id)
        .execute(&state.pool).await?;
    Ok(())
}

#[server(DeleteFooterItem, "/api")]
pub async fn delete_footer_item(id: i32) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("DELETE FROM footer_items WHERE id = $1").bind(id).execute(&state.pool).await?;
    Ok(())
}

#[component]
pub fn Footer() -> impl IntoView {
    let footer_resource = create_resource(|| (), |_| get_footer_items());

    view! {
        <footer class="w-full border-t border-outline-variant/30 py-8 px-6 md:px-[8.5rem] bg-surface-container-low mt-auto flex flex-col md:flex-row justify-between items-center text-xs jetbrains text-outline gap-6 md:gap-0">
            <div class="flex items-center space-x-4">
                <span>"© 2026 RUUD SALYM ERIE. ALL RIGHTS RESERVED."</span>
                <span class="hidden md:inline text-on-surface-variant">"|"</span>
                <span class="hidden md:inline text-surface-variant font-bold text-outline">"OPLYST INTERNATIONAL, LLC."</span>
            </div>
            
            <div class="flex flex-wrap justify-center items-center gap-6">
                <Suspense fallback=move || view! { <div class="w-24 h-4 bg-outline-variant/20 animate-pulse rounded"></div> }>
                    {move || {
                        let items = footer_resource.get().unwrap_or(Ok(vec![])).unwrap_or_default();
                        items.into_iter().map(|item| {
                            view! {
                                <a href=item.href.clone().unwrap_or_else(|| "#".to_string()) class="text-slate-500 dark:text-slate-400 font-medium hover:text-primary transition-colors tracking-widest uppercase text-[0.65rem]">
                                    {item.label.clone()}
                                </a>
                            }
                        }).collect_view()
                    }}
                </Suspense>
            </div>

            <div class="flex items-center space-x-3">
                <span class="hidden md:inline text-[0.65rem] tracking-widest uppercase text-on-surface-variant">"Engineered natively in"</span>
                <a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer" class="flex items-center opacity-70 hover:opacity-100 transition-opacity p-2 bg-surface-container hover:bg-surface-container-high rounded-sm">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" alt="Rust Logo" class="h-5 w-5 dark:invert" />
                    <span class="ml-2 font-bold text-on-surface tracking-widest">"RUST"</span>
                </a>
            </div>
        </footer>
    }
}
