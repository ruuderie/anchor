use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CertRecord {
    pub id: i32,
    pub date_range: String,
    pub title: String,
    pub is_training: bool,
}

#[server(GetCertifications, "/api")]
pub async fn get_certifications() -> Result<Vec<CertRecord>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT id, date_range, title, is_training FROM certifications ORDER BY id ASC")
        .fetch_all(&state.pool)
        .await?;
        
    let certs = rows.into_iter().map(|row| CertRecord {
        id: row.get("id"),
        date_range: row.get("date_range"),
        title: row.get("title"),
        is_training: row.get("is_training"),
    }).collect();
    
    Ok(certs)
}

#[server(AddCertification, "/api")]
pub async fn add_certification(date_range: String, title: String, is_training: bool) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("INSERT INTO certifications (date_range, title, is_training) VALUES ($1, $2, $3)").bind(date_range).bind(title).bind(is_training).execute(&state.pool).await?;
    Ok(())
}

#[server(UpdateCertification, "/api")]
pub async fn update_certification(id: i32, date_range: String, title: String, is_training: bool) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("UPDATE certifications SET date_range = $1, title = $2, is_training = $3 WHERE id = $4").bind(date_range).bind(title).bind(is_training).bind(id).execute(&state.pool).await?;
    Ok(())
}

#[server(DeleteCertification, "/api")]
pub async fn delete_certification(id: i32) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("DELETE FROM certifications WHERE id = $1").bind(id).execute(&state.pool).await?;
    Ok(())
}

#[component]
pub fn Certifications() -> impl IntoView {
    let certs_resource = create_resource(|| (), |_| async move {
        get_certifications().await.unwrap_or_else(|_| vec![])
    });

    view! {
        <main class="pt-32 pb-24 px-6 md:px-[8.5rem] bg-surface min-h-screen">
            <header class="mb-24 flex flex-col items-start max-w-4xl border-b-2 border-outline-variant/30 pb-8">
                <div class="inline-block bg-tertiary-container/20 px-3 py-1 mb-6">
                    <span class="font-label text-[0.6875rem] text-tertiary font-bold tracking-tighter">"INDEX_REF_07 // VERIFIED INFRA"</span>
                </div>
                <h1 class="text-5xl md:text-7xl font-extrabold text-on-surface tracking-[-0.02em] leading-none mb-4 uppercase">
                    "Certifications"
                </h1>
                <p class="text-xl text-on-surface-variant font-medium mt-4 max-w-2xl">
                    "Cryptographically and institutionally verified architecture authorizations."
                </p>
            </header>

            <Suspense fallback=move || view! { <div class="text-on-surface-variant font-bold jetbrains uppercase">"Verifying cryptographic tokens..."</div> }>
                {move || {
                    let certs = certs_resource.get().unwrap_or_default();
                    let salesforce_certs: Vec<_> = certs.iter().filter(|c| !c.is_training).cloned().collect();
                    let training: Vec<_> = certs.iter().filter(|c| c.is_training).cloned().collect();

                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-12 max-w-6xl">
                            <section>
                                <h2 class="text-2xl font-extrabold text-primary mb-8 border-l-4 border-secondary pl-4 uppercase">"Salesforce Architect Credentials"</h2>
                                <div class="space-y-4">
                                    {salesforce_certs.into_iter().enumerate().map(|(idx, cert)| view! {
                                        <div class="bg-surface-container p-6 relative flex items-start group hover:bg-surface-container-high transition-colors cursor-default border-b border-r border-outline-variant/30 shadow-none ring-0">
                                            <div class="absolute top-0 right-0 bg-secondary/10 px-2 py-0.5 text-[0.5rem] font-bold text-secondary jetbrains uppercase">
                                                {format!("ID_AUTH_{:03}", idx + 1)}
                                            </div>
                                            <span class="material-symbols-outlined text-secondary mr-4 text-3xl group-hover:scale-110 transition-transform">"verified_user"</span>
                                            <div>
                                                <h3 class="text-sm font-bold text-on-surface pt-1">{cert.title}</h3>
                                                <span class="text-xs text-outline">{cert.date_range}</span>
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            </section>

                            <section>
                                <h2 class="text-2xl font-extrabold text-primary mb-8 border-l-4 border-secondary pl-4 uppercase">"Executive Training / AI"</h2>
                                <div class="space-y-4">
                                    {training.into_iter().enumerate().map(|(idx, cert)| view! {
                                        <div class="bg-surface-container p-6 relative flex items-start group hover:bg-surface-container-high transition-colors cursor-default border-b border-r border-outline-variant/30 shadow-none ring-0">
                                            <div class="absolute top-0 right-0 bg-primary/10 px-2 py-0.5 text-[0.5rem] font-bold text-primary jetbrains uppercase">
                                                {format!("EXEC_{:03}", idx + 1)}
                                            </div>
                                            <span class="material-symbols-outlined text-primary mr-4 text-3xl group-hover:scale-110 transition-transform">"model_training"</span>
                                            <div>
                                                <h3 class="text-sm font-bold text-on-surface pt-1">{cert.title}</h3>
                                                <span class="text-xs text-outline">{cert.date_range}</span>
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>

                                <div class="mt-16 bg-surface-container-low p-8 border-l-4 border-outline">
                                    <h3 class="text-xl font-bold text-on-surface mb-2 uppercase">"Publications"</h3>
                                    <p class="text-sm font-bold text-on-surface-variant mb-6">"Experience ADITL Podcast | A Day In The Life of a Salesforce Technical Architect"</p>
                                    <a href="https://youtube.com/watch?v=wQ2uO4Xw2Ww" target="_blank" class="inline-flex items-center gap-2 px-4 py-2 bg-on-surface text-surface text-xs font-bold uppercase jetbrains hover:bg-secondary cursor-pointer transition-colors shadow-none text-center">
                                        <span class="material-symbols-outlined text-sm">"play_circle"</span>
                                        "Play Podcast"
                                    </a>
                                </div>
                            </section>
                        </div>
                    }
                }}
            </Suspense>
        </main>
    }
}
