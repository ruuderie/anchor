use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JobRecord {
    pub id: i32,
    pub date_range: String,
    pub role: String,
    pub company: String,
    pub bullets: Vec<String>,
    pub is_client_project: bool,
}

#[server(GetJobs, "/api")]
pub async fn get_jobs() -> Result<Vec<JobRecord>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT id, date_range, role, company, bullets, is_client_project FROM jobs ORDER BY id ASC")
        .fetch_all(&state.pool)
        .await?;
        
    let jobs = rows.into_iter().map(|row| JobRecord {
        id: row.get("id"),
        date_range: row.get("date_range"),
        role: row.get("role"),
        company: row.get("company"),
        bullets: row.get::<Vec<String>, _>("bullets"),
        is_client_project: row.get("is_client_project"),
    }).collect();
    
    Ok(jobs)
}

#[component]
pub fn Resume() -> impl IntoView {
    let download_pdf = create_action(|_: &()| {
        use crate::resume_engine::download_resume;
        async move {
            let _res = download_resume().await;
        }
    });

    let jobs_resource = create_resource(|| (), |_| async move {
        get_jobs().await.unwrap_or_else(|_| vec![])
    });

    view! {
        <main class="pt-32 pb-24 px-6 md:px-[8.5rem] bg-surface-container-low min-h-screen">
            <header class="mb-24 flex flex-col md:flex-row justify-between md:items-end max-w-4xl border-b-2 border-outline-variant/30 pb-8">
                <div>
                    <div class="inline-block bg-secondary-container/20 px-3 py-1 mb-6">
                        <span class="font-label text-[0.6875rem] text-secondary font-bold tracking-tighter">"INDEX_REF_04 // CV"</span>
                    </div>
                    <h1 class="text-5xl md:text-7xl font-extrabold text-primary tracking-[-0.02em] leading-none mb-4">
                        "EXPERIENCE LOG"
                    </h1>
                </div>

                <div class="mt-8 md:mt-0 flex flex-col items-start md:items-end">
                    <button
                        on:click=move |_| download_pdf.dispatch(())
                        class="bg-secondary shadow-none border-none outline-none text-white px-6 py-3 font-label text-sm tracking-widest font-bold uppercase hover:bg-on-secondary-fixed transition-colors flex items-center gap-2 rounded-none"
                    >
                        <span class="material-symbols-outlined">"download"</span>
                        "GET_PDF"
                    </button>
                    <div class="text-[0.6rem] text-outline mt-2 jetbrains text-left md:text-right">"GENERATES LATEX >> PDF VIA TECTONIC"</div>
                </div>
            </header>

            <Suspense fallback=move || view! { <div class="text-on-surface-variant font-bold jetbrains uppercase">"Hydrating systems database..."</div> }>
                {move || {
                    let jobs = jobs_resource.get().unwrap_or_default();
                    let client_projects: Vec<_> = jobs.iter().filter(|j| j.is_client_project).cloned().collect();
                    let employment: Vec<_> = jobs.iter().filter(|j| !j.is_client_project).cloned().collect();

                    view! {
                        <div class="max-w-4xl space-y-24">
                            <div>
                                <h2 class="text-3xl font-extrabold text-on-surface mb-12 border-l-4 border-secondary pl-4 uppercase">"Project Experience"</h2>
                                <div class="space-y-16">
                                    {client_projects.into_iter().map(|job| view! {
                                        <section class="grid grid-cols-1 md:grid-cols-12 gap-4 md:gap-8">
                                            <div class="md:col-span-3 font-label text-xs sm:text-sm text-outline font-bold pt-1 uppercase tracking-widest">{job.date_range}</div>
                                            <div class="md:col-span-9 bg-surface-container p-6 md:p-8 blueprint-overlay shadow-none border-0 ring-0">
                                                <h3 class="text-xl md:text-2xl font-bold text-primary mb-1">{job.role}</h3>
                                                <div class="text-secondary font-medium mb-6">{job.company}</div>
                                                <ul class="text-on-surface-variant leading-relaxed text-sm space-y-3 list-none p-0 m-0">
                                                    {job.bullets.into_iter().map(|b| view! {
                                                        <li class="relative pl-4 before:content-['>'] before:absolute before:-left-1 before:text-secondary before:font-bold">
                                                            {b}
                                                        </li>
                                                    }).collect_view()}
                                                </ul>
                                            </div>
                                        </section>
                                    }).collect_view()}
                                </div>
                            </div>

                            <div>
                                <h2 class="text-3xl font-extrabold text-on-surface mb-12 border-l-4 border-secondary pl-4 uppercase">"Employment History"</h2>
                                <div class="space-y-16">
                                    {employment.into_iter().map(|job| view! {
                                        <section class="grid grid-cols-1 md:grid-cols-12 gap-4 md:gap-8">
                                            <div class="md:col-span-3 font-label text-xs sm:text-sm text-outline font-bold pt-1 uppercase tracking-widest">{job.date_range}</div>
                                            <div class="md:col-span-9 bg-surface-container p-6 md:p-8 blueprint-overlay shadow-none border-0 ring-0">
                                                <h3 class="text-xl md:text-2xl font-bold text-primary mb-1">{job.role}</h3>
                                                <div class="text-secondary font-medium mb-6">{job.company}</div>
                                                <ul class="text-on-surface-variant leading-relaxed text-sm space-y-3 list-none p-0 m-0">
                                                    {job.bullets.into_iter().map(|b| view! {
                                                        <li class="relative pl-4 before:content-['>'] before:absolute before:-left-1 before:text-secondary before:font-bold">
                                                            {b}
                                                        </li>
                                                    }).collect_view()}
                                                </ul>
                                            </div>
                                        </section>
                                    }).collect_view()}
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Suspense>
        </main>
    }
}
