use leptos::*;

use crate::pages::certifications::get_certifications;
use crate::pages::projects::get_projects;
use crate::pages::resume::get_jobs;
use crate::auth::*;
use crate::components::admin_modal::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/public/webauthn.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn registerDevice(optionsJson: &str) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(catch)]
    async fn authenticateDevice(optionsJson: &str) -> Result<JsValue, JsValue>;
}

#[component]
pub fn Admin() -> impl IntoView {
    let (is_authenticated, set_authenticated) = create_signal(false);
    let (active_tab, set_active_tab) = create_signal("DASHBOARD");
    let (username, set_username) = create_signal(String::new());

    let (modal_state, set_modal_state) = create_signal(ModalState::None);
    provide_context(modal_state);
    provide_context(set_modal_state);
    
    let (refresh, set_refresh) = create_signal(0);
    provide_context(refresh);
    provide_context(set_refresh);

    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(true) = check_session().await {
                set_authenticated.set(true);
            }
        });
    });

    let login_action = create_action(move |_: &()| async move {
        let uname = username.get_untracked();
        if uname.is_empty() { return; }
        
        if let Ok(_payload) = login_start(uname.clone()).await {
            #[cfg(target_arch = "wasm32")]
            {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&_payload) {
                    if let (Some(c_str), Some(o_str)) = (val["challenge_id"].as_str(), val["options"].as_str()) {
                        if let Ok(challenge_id) = uuid::Uuid::parse_str(c_str) {
                            if let Ok(cred_js) = authenticateDevice(o_str).await {
                                if let Some(cred_str) = cred_js.as_string() {
                                    if let Ok(_) = login_finish(uname, challenge_id, cred_str).await {
                                        set_authenticated.set(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    let register_action = create_action(move |_: &()| async move {
        let uname = username.get_untracked();
        if uname.is_empty() { return; }
        
        if let Ok(_payload) = register_start(uname.clone()).await {
            #[cfg(target_arch = "wasm32")]
            {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&_payload) {
                    if let (Some(c_str), Some(o_str)) = (val["challenge_id"].as_str(), val["options"].as_str()) {
                        if let Ok(challenge_id) = uuid::Uuid::parse_str(c_str) {
                            if let Ok(cred_js) = registerDevice(o_str).await {
                                if let Some(cred_str) = cred_js.as_string() {
                                    if let Ok(_) = register_finish(uname, challenge_id, cred_str).await {
                                        set_authenticated.set(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    view! {
        <main class="min-h-screen bg-surface-container-low text-on-surface flex flex-col pt-24 px-6 md:px-[8.5rem]">
            {move || if !is_authenticated.get() {
                view! {
                    <div class="flex-1 flex justify-center items-center">
                        <div class="w-full max-w-lg bg-surface-container-highest p-1 lg:p-1 blueprint-overlay">
                            <div class="bg-surface-container-lowest p-12">
                                <div class="inline-block bg-secondary-container/20 px-3 py-1 mb-6">
                                    <span class="font-label text-[0.6875rem] text-secondary font-bold tracking-tighter">"SECURE_ZONE // 0xAUTH"</span>
                                </div>
                                <h2 class="text-4xl font-extrabold text-primary mb-8 tracking-tight">"SYSTEM_CMS"</h2>

                                <div class="space-y-12">
                                    <div class="relative w-full group">
                                        <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline text-left block mb-2">"Identity Hash"</label>
                                        <input 
                                            type="text" 
                                            placeholder="admin" 
                                            on:input=move |ev| set_username.set(event_target_value(&ev))
                                            prop:value=username
                                            class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-4 jetbrains text-lg text-on-surface transition-all placeholder:text-outline-variant/50" 
                                        />
                                    </div>

                                    <div class="space-y-4">
                                        <button 
                                            on:click=move |_| login_action.dispatch(())
                                            class="w-full bg-primary text-white py-6 jetbrains font-bold text-sm tracking-[0.2em] uppercase hover:bg-primary-container transition-colors"
                                        >
                                            "Authenticate // Passkey"
                                        </button>

                                        <button 
                                            on:click=move |_| register_action.dispatch(())
                                            class="w-full border border-primary/20 text-primary py-4 jetbrains font-bold text-sm tracking-[0.2em] uppercase hover:bg-surface-container transition-colors"
                                        >
                                            "Register Device"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="flex-1 flex flex-col md:flex-row gap-12 pb-24">
                        // Sidebar
                        <aside class="w-full md:w-64 shrink-0 space-y-2">
                            <div class="mb-12">
                                <span class="font-label text-[0.6875rem] text-outline font-bold tracking-widest uppercase block mb-4">"Navigation"</span>
                                <div class="space-y-1">
                                    <div class="flex flex-col space-y-4">
                            {["DASHBOARD", "MAILING LIST", "SETTINGS", "RESUME", "PROJECTS", "CERTIFICATIONS", "BLOG", "PROFILES", "SECURITY"].iter().map(|&t| {
                                            let tab = t; // Capture `t` for the closure
                                            view! {
                                                <button 
                                                    on:click=move |_| set_active_tab.set(tab)
                                                    class=move || format!(
                                                        "w-full text-left px-4 py-3 jetbrains text-sm font-bold tracking-wider transition-colors {}",
                                                        if active_tab.get() == tab {
                                                            "bg-primary text-on-primary"
                                                        } else {
                                                            "text-slate-500 hover:bg-surface-container-high dark:text-slate-400 dark:hover:bg-slate-800"
                                                        }
                                                    )
                                                >
                                                    {tab}
                                                </button>
                                            }
                                        }
                                    ).collect_view()}
                                </div>
                            </div>
                            </div>
                            
                            <button 
                                on:click=move |_| set_authenticated.set(false)
                                class="text-error text-xs jetbrains font-bold uppercase tracking-widest hover:underline"
                            >
                                "[ TERMINATE SESSION ]"
                            </button>
                        </aside>

                        // Main Content Area
                        <section class="flex-1 bg-surface-container-highest p-1 blueprint-overlay min-h-[600px]">
                            <div class="bg-surface-container-lowest h-full p-8 md:p-12 relative flex flex-col">
                                // Header
                                <div class="flex justify-between items-end border-b-2 border-outline-variant pb-6 mb-8">
                                    <div>
                                        <div class="inline-block bg-secondary-container/20 px-3 py-1 mb-4">
                                            <span class="font-label text-[0.6875rem] text-secondary font-bold tracking-tighter">"DATABASE // LIVE"</span>
                                        </div>
                                        <h2 class="text-3xl font-extrabold text-primary tracking-tight uppercase">
                                            {move || active_tab.get()}
                                        </h2>
                                    </div>
                                    <button 
                                        on:click=move |_| {
                                            let state = match active_tab.get() {
                                                "SETTINGS" => ModalState::Settings,
                                                "RESUME" => ModalState::Job(None),
                                                "PROJECTS" => ModalState::Project(None),
                                                "CERTIFICATIONS" => ModalState::Cert(None),
                                                "BLOG" => ModalState::Post(None),
                                                "PROFILES" => ModalState::Profile(None),
                                                "SECURITY" => ModalState::Passkey,
                                                _ => ModalState::None,
                                            };
                                            set_modal_state.set(state);
                                        }
                                        class="bg-primary text-on-primary px-8 py-4 jetbrains text-xs font-bold tracking-[0.2em] uppercase hover:bg-primary-container transition-colors"
                                    >
                                        {move || if active_tab.get() == "SETTINGS" { "EDIT VALUES" } else if active_tab.get() == "DASHBOARD" || active_tab.get() == "MAILING LIST" { "REFRESH" } else { "NEW ENTRY +" }}
                                    </button>
                                </div>

                                // Datagrid View
                                <div class="flex-1 overflow-x-auto">
                                    {move || match active_tab.get() {
                                        "DASHBOARD" => view! { <DashboardView /> }.into_view(),
                                        "MAILING LIST" => view! { <MailingListTable /> }.into_view(),
                                        "SETTINGS" => view! { <SettingsReadView /> }.into_view(),
                                        "RESUME" => view! { <JobTable /> }.into_view(),
                                        "PROJECTS" => view! { <ProjectTable /> }.into_view(),
                                        "CERTIFICATIONS" => view! { <CertTable /> }.into_view(),
                                        "BLOG" => view! { <PostTable /> }.into_view(),
                                        "PROFILES" => view! { <ResumeProfileTable /> }.into_view(),
                                        "SECURITY" => view! { <PasskeyTable /> }.into_view(),
                                        _ => view! { 
                                            <div class="h-64 flex items-center justify-center border-2 border-dashed border-outline-variant text-outline">
                                                <span class="jetbrains text-sm">"MODULE_OFFLINE"</span>
                                            </div> 
                                        }.into_view(),
                                    }}
                                </div>
                            </div>
                            <AdminEditorModal />
                        </section>
                    </div>
                }.into_view()
            }}
        </main>
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DashboardStats {
    pub mempool_requests_24h: i64,
    pub total_signups: i64,
    pub total_mailing_list: i64,
    pub recent_page_views: i64,
}

#[server(GetDashboardStats, "/api")]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    
    let mempool: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM api_requests_log WHERE endpoint = 'mempool_api' AND created_at > NOW() - INTERVAL '24 hours'").fetch_one(&state.pool).await.unwrap_or(0);
    let signups: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users").fetch_one(&state.pool).await.unwrap_or(0);
    let mailing: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM mailing_list").fetch_one(&state.pool).await.unwrap_or(0);
    let views: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM page_views WHERE created_at > NOW() - INTERVAL '24 hours'").fetch_one(&state.pool).await.unwrap_or(0);
    
    Ok(DashboardStats {
        mempool_requests_24h: mempool,
        total_signups: signups,
        total_mailing_list: mailing,
        recent_page_views: views,
    })
}

#[component]
fn DashboardView() -> impl IntoView {
    let refresh = expect_context::<ReadSignal<i32>>();
    let stats_resource = create_resource(move || refresh.get(), |_| get_dashboard_stats());

    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"COMPILING_TELEMETRY..."</div> }>
            {move || match stats_resource.get() {
                Some(Ok(stats)) => view! {
                    <div class="grid grid-cols-2 gap-8">
                        <div class="p-8 border border-outline-variant/30 flex flex-col justify-between">
                            <span class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider mb-4">"Mempool Network Fetches (24H)"</span>
                            <span class="text-5xl font-extrabold text-[#f7931a]">{stats.mempool_requests_24h}</span>
                        </div>
                        <div class="p-8 border border-outline-variant/30 flex flex-col justify-between">
                            <span class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider mb-4">"Page Views (24H)"</span>
                            <span class="text-5xl font-extrabold text-primary">{stats.recent_page_views}</span>
                        </div>
                        <div class="p-8 border border-outline-variant/30 flex flex-col justify-between">
                            <span class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider mb-4">"Total Mailing List & Leads"</span>
                            <span class="text-5xl font-extrabold text-secondary">{stats.total_mailing_list}</span>
                        </div>
                        <div class="p-8 border border-outline-variant/30 flex flex-col justify-between">
                            <span class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider mb-4">"Admin Identities Registered"</span>
                            <span class="text-5xl font-extrabold text-on-surface">{stats.total_signups}</span>
                        </div>
                    </div>
                }.into_view(),
                _ => view! { <div class="text-error">"Failed to load settings"</div> }.into_view()
            }}
        </Transition>
    }
}

#[component]
fn SettingsReadView() -> impl IntoView {
    let refresh = expect_context::<ReadSignal<i32>>();
    let settings_res = create_resource(move || refresh.get(), |_| crate::pages::landing::get_site_settings());

    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"LOADING SETTINGS..."</div> }>
            {move || match settings_res.get() {
                Some(Ok(s)) => view! {
                    <div class="grid grid-cols-1 gap-4 text-left jetbrains text-sm">
                        
                        <div class="grid grid-cols-3 border-b-2 border-outline-variant/30 pb-2 mb-4">
                            <div class="font-label text-[0.65rem] uppercase tracking-widest text-outline">"KEY"</div>
                            <div class="col-span-2 font-label text-[0.65rem] uppercase tracking-widest text-outline">"VALUE"</div>
                        </div>

                        // Hero
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-outline-variant uppercase tracking-widest text-xs">"CURRENT FOCUS"</div>
                            <div class="col-span-2 text-on-surface font-medium">{&s.current_focus}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-outline-variant uppercase tracking-widest text-xs">"STATUS"</div>
                            <div class="col-span-2 text-on-surface font-medium"><div class="inline-flex items-center gap-2"><div class="w-2 h-2 rounded-full" style=format!("background-color: {};", s.status_color)></div>{&s.status}</div></div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-outline-variant uppercase tracking-widest text-xs">"SUBTITLE"</div>
                            <div class="col-span-2 text-on-surface font-medium truncate">{&s.hero_subtitle}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-outline-variant uppercase tracking-widest text-xs">"QUOTE"</div>
                            <div class="col-span-2 text-on-surface font-medium truncate">{&s.hero_quote}</div>
                        </div>

                        // Global
                        <div class="grid grid-cols-3 py-2 border-b border-primary/20 hover:bg-surface-container/30 mt-4">
                            <div class="text-primary uppercase tracking-widest text-xs">"SITE TITLE"</div>
                            <div class="col-span-2 text-on-surface font-bold">{&s.site_title}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-primary uppercase tracking-widest text-xs">"WEBHOOK URL"</div>
                            <div class="col-span-2 text-on-surface font-mono text-xs">{&s.webhook_url}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-primary uppercase tracking-widest text-xs">"ADMIN NOTIFICATION"</div>
                            <div class="col-span-2 text-on-surface font-mono text-xs">{&s.admin_email}</div>
                        </div>

                        // Lead Capture
                        <div class="grid grid-cols-3 py-2 border-b border-secondary/20 hover:bg-surface-container/30 mt-4">
                            <div class="text-secondary uppercase tracking-widest text-xs">"LC TITLE"</div>
                            <div class="col-span-2 text-on-surface font-medium">{&s.lc_title}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-secondary uppercase tracking-widest text-xs">"LC DESC"</div>
                            <div class="col-span-2 text-on-surface font-medium truncate">{&s.lc_desc}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-secondary uppercase tracking-widest text-xs">"LC BTN"</div>
                            <div class="col-span-2 text-on-surface font-medium">{&s.lc_btn}</div>
                        </div>
                        
                        // Real Estate
                        <div class="grid grid-cols-3 py-2 border-b border-primary/20 hover:bg-surface-container/30 mt-4">
                            <div class="text-primary uppercase tracking-widest text-xs">"RE TITLE"</div>
                            <div class="col-span-2 text-on-surface font-medium">{&s.real_estate_title}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-primary uppercase tracking-widest text-xs">"RE DESC"</div>
                            <div class="col-span-2 text-on-surface font-medium truncate">{&s.real_estate_desc}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-primary uppercase tracking-widest text-xs">"RE LC TITLE"</div>
                            <div class="col-span-2 text-on-surface font-medium">{&s.re_lc_title}</div>
                        </div>
                        <div class="grid grid-cols-3 py-2 border-b border-outline-variant/10 hover:bg-surface-container/30">
                            <div class="text-primary uppercase tracking-widest text-xs">"RE LC OPTIONS"</div>
                            <div class="col-span-2 text-on-surface font-mono text-xs truncate">{&s.re_options_json}</div>
                        </div>
                        
                    </div>
                }.into_view(),
                _ => view! { <div class="text-error">"Failed to load settings"</div> }.into_view()
            }}
        </Transition>
    }
}

#[component]
fn ResumeProfileTable() -> impl IntoView {
    use crate::resume_engine::{get_resume_profiles, delete_resume_profile, download_resume};
    let refresh = expect_context::<ReadSignal<i32>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let set_modal_state = expect_context::<WriteSignal<crate::components::admin_modal::ModalState>>();
    
    let items_res = create_resource(move || refresh.get(), |_| get_resume_profiles());

    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"LOADING..."</div> }>
            <table class="w-full text-left border-collapse">
                <thead>
                    <tr class="border-b-2 border-outline-variant/30">
                        <th class="py-4 font-label text-[0.65rem] uppercase tracking-widest text-outline">"ID"</th>
                        <th class="py-4 font-label text-[0.65rem] uppercase tracking-widest text-outline">"PROFILE NAME"</th>
                        <th class="py-4 font-label text-[0.65rem] uppercase tracking-widest text-outline text-right">"ACTIONS"</th>
                    </tr>
                </thead>
                <tbody class="jetbrains text-sm">
                    {move || match items_res.get() {
                        Some(Ok(items)) => items.into_iter().map(|item| {
                            let id_val = item.id;
                            let clone_item = item.clone();
                            view! {
                                <tr class="border-b border-outline-variant/10 hover:bg-surface-container/50 transition-colors">
                                    <td class="py-4 text-outline-variant font-medium">#{id_val}</td>
                                    <td class="py-4 font-bold text-on-surface">{&item.name}</td>
                                    <td class="py-4 text-right space-x-4">
                                        <button 
                                            on:click=move |_| {
                                                spawn_local(async move {
                                                    if let Ok(bytes) = download_resume(id_val).await.map_err(|e| format!("{:?}", e)) {
                                                        use base64::{Engine as _, engine::general_purpose::STANDARD};
                                                        let b64 = STANDARD.encode(&bytes);
                                                        let url = format!("data:application/pdf;base64,{}", b64);
                                                        
                                                        let document = leptos::document();
                                                        if let Ok(a) = document.create_element("a") {
                                                            let _ = a.set_attribute("href", &url);
                                                            let _ = a.set_attribute("download", &format!("ruuderie_resume_profile_{}.pdf", id_val));
                                                            use web_sys::wasm_bindgen::JsCast;
                                                            let html_a = a.unchecked_into::<web_sys::HtmlElement>();
                                                            html_a.click();
                                                        }
                                                    }
                                                });
                                            }
                                            class="text-primary hover:text-primary-container font-medium tracking-wide"
                                        >"[PDF]"</button>
                                        <button on:click=move |_| set_modal_state.set(crate::components::admin_modal::ModalState::Profile(Some(clone_item.clone()))) class="text-secondary hover:text-on-secondary-fixed-variant font-medium tracking-wide">"[EDIT]"</button>
                                        <button 
                                            on:click=move |_| {
                                                spawn_local(async move {
                                                    let _ = delete_resume_profile(id_val).await;
                                                    set_refresh.set(refresh.get_untracked() + 1);
                                                });
                                            }
                                            class="text-error hover:text-error/80 font-medium tracking-wide"
                                        >"[DEL]"</button>
                                    </td>
                                </tr>
                            }
                        }).collect_view(),
                        _ => view! { <tr><td colspan="3" class="py-8 text-center text-error">"ERR_NO_DATA"</td></tr> }.into_view(),
                    }}
                </tbody>
            </table>
        </Transition>
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MailingListRecord {
    pub id: i32,
    pub email: String,
    pub list_type: String,
    pub preferences: String,
    pub created_at: String,
}

#[server(GetMailingList, "/api")]
pub async fn get_mailing_list() -> Result<Vec<MailingListRecord>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    
    let rows = sqlx::query("SELECT id, email, list_type, preferences::text as prefs, created_at FROM mailing_list ORDER BY created_at DESC").fetch_all(&state.pool).await?;
    
    let mut records = Vec::new();
    for row in rows {
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
        records.push(MailingListRecord {
            id: row.get("id"),
            email: row.get("email"),
            list_type: row.get("list_type"),
            preferences: row.get("prefs"),
            created_at: created_at.format("%Y-%m-%d %H:%M").to_string(),
        });
    }
    
    Ok(records)
}

#[component]
fn MailingListTable() -> impl IntoView {
    let refresh = expect_context::<ReadSignal<i32>>();
    let list_resource = create_resource(move || refresh.get(), |_| get_mailing_list());
    
    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"QUERYING_DB..."</div> }>
            <table class="w-full text-left jetbrains text-sm">
                <thead>
                    <tr class="text-outline border-b border-outline-variant/30">
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Email"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Type"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Preferences"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Timestamp"</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-outline-variant/20">
                    {move || match list_resource.get() {
                        Some(Ok(items)) => items.into_iter().map(|i| view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 font-bold text-primary">{i.email}</td>
                                <td class="py-4 px-4 text-on-surface">{i.list_type}</td>
                                <td class="py-4 px-4 text-outline truncate max-w-[200px]" title=i.preferences.clone()>{i.preferences}</td>
                                <td class="py-4 px-4 text-outline-variant">{i.created_at}</td>
                            </tr>
                        }).collect_view(),
                        _ => view! { <tr><td colspan="4" class="py-8 text-center text-error">"ERR_NO_DATA"</td></tr> }.into_view(),
                    }}
                </tbody>
            </table>
        </Transition>
    }
}

#[component]
fn JobTable() -> impl IntoView {
    let refresh = expect_context::<ReadSignal<i32>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    
    let jobs_resource = create_resource(move || refresh.get(), |_| get_jobs());
    
    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"QUERYING_DB..."</div> }>
            <table class="w-full text-left jetbrains text-sm">
                <thead>
                    <tr class="text-outline border-b border-outline-variant/30">
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"ID"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Company"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Role"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Actions"</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-outline-variant/20">
                    {move || match jobs_resource.get() {
                        Some(Ok(jobs)) => jobs.into_iter().map(|j| {
                            let job_clone = j.clone();
                            view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 text-outline-variant">"#" {j.id}</td>
                                <td class="py-4 px-4 font-bold text-primary">{j.company}</td>
                                <td class="py-4 px-4 text-on-surface">{j.role}</td>
                                <td class="py-4 px-4">
                                    <div class="flex space-x-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button 
                                            on:click=move |_| set_modal_state.set(ModalState::Job(Some(job_clone.clone())))
                                            class="text-secondary hover:underline uppercase text-xs"
                                        >
                                            "Edit"
                                        </button>
                                        <button 
                                            on:click=move |_| {
                                                let id = j.id;
                                                spawn_local(async move {
                                                    if let Ok(_) = crate::pages::resume::delete_job(id).await {
                                                        set_refresh.set(refresh.get_untracked() + 1);
                                                    }
                                                });
                                            }
                                            class="text-error hover:underline uppercase text-xs"
                                        >
                                            "Drop"
                                        </button>
                                    </div>
                                </td>
                            </tr>
                            }
                        }).collect_view(),
                        _ => view! { <tr><td colspan="4" class="py-8 text-center text-error">"ERR_NO_DATA"</td></tr> }.into_view(),
                    }}
                </tbody>
            </table>
        </Transition>
    }
}

#[component]
fn ProjectTable() -> impl IntoView {
    let refresh = expect_context::<ReadSignal<i32>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let projects_resource = create_resource(move || refresh.get(), |_| get_projects());
    
    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"QUERYING_DB..."</div> }>
            <table class="w-full text-left jetbrains text-sm">
                <thead>
                    <tr class="text-outline border-b border-outline-variant/30">
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"ID"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Title"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Impact"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Actions"</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-outline-variant/20">
                    {move || match projects_resource.get() {
                        Some(Ok(projects)) => projects.into_iter().map(|p| {
                            let p_clone = p.clone();
                            view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 text-outline-variant">"#" {p.id}</td>
                                <td class="py-4 px-4 font-bold text-primary">{p.title}</td>
                                <td class="py-4 px-4 text-on-surface truncate max-w-[200px]">{p.impact}</td>
                                <td class="py-4 px-4">
                                    <div class="flex space-x-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button 
                                            on:click=move |_| set_modal_state.set(ModalState::Project(Some(p_clone.clone())))
                                            class="text-secondary hover:underline uppercase text-xs"
                                        >
                                            "Edit"
                                        </button>
                                        <button 
                                            on:click=move |_| {
                                                let id = p.id;
                                                spawn_local(async move {
                                                    if let Ok(_) = crate::pages::projects::delete_project(id).await {
                                                        set_refresh.set(refresh.get_untracked() + 1);
                                                    }
                                                });
                                            }
                                            class="text-error hover:underline uppercase text-xs"
                                        >
                                            "Drop"
                                        </button>
                                    </div>
                                </td>
                            </tr>
                            }
                        }).collect_view(),
                        _ => view! { <tr><td colspan="4" class="py-8 text-center text-error">"ERR_NO_DATA"</td></tr> }.into_view(),
                    }}
                </tbody>
            </table>
        </Transition>
    }
}

#[component]
fn CertTable() -> impl IntoView {
    let refresh = expect_context::<ReadSignal<i32>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let certs_resource = create_resource(move || refresh.get(), |_| get_certifications());
    
    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"QUERYING_DB..."</div> }>
            <table class="w-full text-left jetbrains text-sm">
                <thead>
                    <tr class="text-outline border-b border-outline-variant/30">
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"ID"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Certification"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Date"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Actions"</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-outline-variant/20">
                    {move || match certs_resource.get() {
                        Some(Ok(certs)) => certs.into_iter().map(|c| {
                            let c_clone = c.clone();
                            view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 text-outline-variant">"#" {c.id}</td>
                                <td class="py-4 px-4 font-bold text-primary">{c.title}</td>
                                <td class="py-4 px-4 text-on-surface">{c.date_range}</td>
                                <td class="py-4 px-4">
                                    <div class="flex space-x-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button 
                                            on:click=move |_| set_modal_state.set(ModalState::Cert(Some(c_clone.clone())))
                                            class="text-secondary hover:underline uppercase text-xs"
                                        >
                                            "Edit"
                                        </button>
                                        <button 
                                            on:click=move |_| {
                                                let id = c.id;
                                                spawn_local(async move {
                                                    if let Ok(_) = crate::pages::certifications::delete_certification(id).await {
                                                        set_refresh.set(refresh.get_untracked() + 1);
                                                    }
                                                });
                                            }
                                            class="text-error hover:underline uppercase text-xs"
                                        >
                                            "Drop"
                                        </button>
                                    </div>
                                </td>
                            </tr>
                            }
                        }).collect_view(),
                        _ => view! { <tr><td colspan="4" class="py-8 text-center text-error">"ERR_NO_DATA"</td></tr> }.into_view(),
                    }}
                </tbody>
            </table>
        </Transition>
    }
}

#[component]
fn PostTable() -> impl IntoView {
    use crate::pages::blog::get_posts;
    let refresh = expect_context::<ReadSignal<i32>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let posts_resource = create_resource(move || refresh.get(), |_| get_posts());
    
    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"QUERYING_DB..."</div> }>
            <table class="w-full text-left jetbrains text-sm">
                <thead>
                    <tr class="text-outline border-b border-outline-variant/30">
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"ID"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Slug"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Title"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Actions"</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-outline-variant/20">
                    {move || match posts_resource.get() {
                        Some(Ok(posts)) => posts.into_iter().map(|p| {
                            let p_clone = p.clone();
                            view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 text-outline-variant">"#" {p.id}</td>
                                <td class="py-4 px-4 font-bold text-outline">{p.slug}</td>
                                <td class="py-4 px-4 text-primary font-bold">{p.title}</td>
                                <td class="py-4 px-4">
                                    <div class="flex space-x-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button 
                                            on:click=move |_| set_modal_state.set(ModalState::Post(Some(p_clone.clone())))
                                            class="text-secondary hover:underline uppercase text-xs"
                                        >
                                            "Edit"
                                        </button>
                                        <button 
                                            on:click=move |_| {
                                                let id = p.id;
                                                spawn_local(async move {
                                                    if let Ok(_) = crate::pages::blog::delete_post(id).await {
                                                        set_refresh.set(refresh.get_untracked() + 1);
                                                    }
                                                });
                                            }
                                            class="text-error hover:underline uppercase text-xs"
                                        >
                                            "Drop"
                                        </button>
                                    </div>
                                </td>
                            </tr>
                            }
                        }).collect_view(),
                        _ => view! { <tr><td colspan="4" class="py-8 text-center text-error">"ERR_NO_DATA"</td></tr> }.into_view(),
                    }}
                </tbody>
            </table>
        </Transition>
    }
}

#[component]
fn PasskeyTable() -> impl IntoView {
    use crate::auth::get_users;
    let refresh = expect_context::<ReadSignal<i32>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let users_resource = create_resource(move || refresh.get(), |_| get_users());
    
    view! {
        <Transition fallback=move || view! { <div class="jetbrains text-sm text-outline">"QUERYING_DB..."</div> }>
            <table class="w-full text-left jetbrains text-sm">
                <thead>
                    <tr class="text-outline border-b border-outline-variant/30">
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"ID"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Username"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Created At"</th>
                        <th class="py-4 px-4 font-normal tracking-widest uppercase">"Actions"</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-outline-variant/20">
                    {move || match users_resource.get() {
                        Some(Ok(users)) => users.into_iter().map(|u| view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 text-outline-variant">"#" {u.id}</td>
                                <td class="py-4 px-4 font-bold text-primary">{u.username}</td>
                                <td class="py-4 px-4 text-outline">{u.created_at}</td>
                                <td class="py-4 px-4">
                                    <div class="flex space-x-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button 
                                            on:click=move |_| {
                                                let id = u.id;
                                                spawn_local(async move {
                                                    if let Ok(_) = crate::auth::delete_user(id).await {
                                                        set_refresh.set(refresh.get_untracked() + 1);
                                                    }
                                                });
                                            }
                                            class="text-error hover:underline uppercase text-xs"
                                        >
                                            "Revoke"
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        }).collect_view(),
                        _ => view! { <tr><td colspan="4" class="py-8 text-center text-error">"ERR_NO_DATA"</td></tr> }.into_view(),
                    }}
                </tbody>
            </table>
        </Transition>
    }
}
