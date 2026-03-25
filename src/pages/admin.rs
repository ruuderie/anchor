use leptos::*;

use crate::pages::certifications::{get_certifications, GetCertifications};
use crate::pages::projects::{get_projects, GetProjects};
use crate::pages::resume::{get_jobs, GetJobs};
use crate::auth::*;

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
    let (active_tab, set_active_tab) = create_signal("RESUME");
    let (username, set_username) = create_signal(String::new());

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
                                    <For
                                        each=|| vec!["RESUME", "PROJECTS", "CERTIFICATIONS", "POSTS", "PASSKEYS"]
                                        key=|tab| tab.to_string()
                                        children=move |tab| {
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
                                    />
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
                                    <button class="bg-primary text-on-primary px-8 py-4 jetbrains text-xs font-bold tracking-[0.2em] uppercase hover:bg-primary-container transition-colors">
                                        "NEW ENTRY +"
                                    </button>
                                </div>

                                // Datagrid View
                                <div class="flex-1 overflow-x-auto">
                                    {move || match active_tab.get() {
                                        "RESUME" => view! { <JobTable /> }.into_view(),
                                        "PROJECTS" => view! { <ProjectTable /> }.into_view(),
                                        "CERTIFICATIONS" => view! { <CertTable /> }.into_view(),
                                        _ => view! { 
                                            <div class="h-64 flex items-center justify-center border-2 border-dashed border-outline-variant text-outline">
                                                <span class="jetbrains text-sm">"MODULE_OFFLINE"</span>
                                            </div> 
                                        }.into_view(),
                                    }}
                                </div>
                            </div>
                        </section>
                    </div>
                }.into_view()
            }}
        </main>
    }
}

#[component]
fn JobTable() -> impl IntoView {
    let jobs_resource = create_resource(|| (), |_| get_jobs());
    
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
                        Some(Ok(jobs)) => jobs.into_iter().map(|j| view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 text-outline-variant">"#" {j.id}</td>
                                <td class="py-4 px-4 font-bold text-primary">{j.company}</td>
                                <td class="py-4 px-4 text-on-surface">{j.role}</td>
                                <td class="py-4 px-4">
                                    <div class="flex space-x-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button class="text-secondary hover:underline uppercase text-xs">"Edit"</button>
                                        <button class="text-error hover:underline uppercase text-xs">"Drop"</button>
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

#[component]
fn ProjectTable() -> impl IntoView {
    let projects_resource = create_resource(|| (), |_| get_projects());
    
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
                        Some(Ok(projects)) => projects.into_iter().map(|p| view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 text-outline-variant">"#" {p.id}</td>
                                <td class="py-4 px-4 font-bold text-primary">{p.title}</td>
                                <td class="py-4 px-4 text-on-surface truncate max-w-[200px]">{p.impact}</td>
                                <td class="py-4 px-4">
                                    <div class="flex space-x-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button class="text-secondary hover:underline uppercase text-xs">"Edit"</button>
                                        <button class="text-error hover:underline uppercase text-xs">"Drop"</button>
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

#[component]
fn CertTable() -> impl IntoView {
    let certs_resource = create_resource(|| (), |_| get_certifications());
    
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
                        Some(Ok(certs)) => certs.into_iter().map(|c| view! {
                            <tr class="hover:bg-surface-container-high transition-colors group">
                                <td class="py-4 px-4 text-outline-variant">"#" {c.id}</td>
                                <td class="py-4 px-4 font-bold text-primary">{c.title}</td>
                                <td class="py-4 px-4 text-on-surface">{c.date_range}</td>
                                <td class="py-4 px-4">
                                    <div class="flex space-x-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <button class="text-secondary hover:underline uppercase text-xs">"Edit"</button>
                                        <button class="text-error hover:underline uppercase text-xs">"Drop"</button>
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
