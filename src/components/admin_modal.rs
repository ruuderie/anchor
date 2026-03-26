use leptos::*;

#[derive(Clone, PartialEq, Debug)]
pub enum ModalState {
    None,
    Job(Option<crate::pages::resume::JobRecord>),
    Project(Option<crate::pages::projects::ProjectRecord>),
    Cert(Option<crate::pages::certifications::CertRecord>),
    Post(Option<crate::pages::blog::PostRecord>),
    Profile(Option<crate::resume_engine::ResumeProfile>),
    Passkey,
    Settings,
}

#[component]
pub fn AdminEditorModal() -> impl IntoView {
    let modal_state = expect_context::<ReadSignal<ModalState>>();
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let _set_refresh = expect_context::<WriteSignal<i32>>();
    let _refresh = expect_context::<ReadSignal<i32>>();

    let close_modal = move || set_modal_state.set(ModalState::None);

    view! {
        <Show when=move || modal_state.get() != ModalState::None>
            <div class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm p-6 overflow-y-auto">
                <div class="relative w-full max-w-4xl bg-surface-container-highest p-1 blueprint-overlay max-h-[90vh] flex flex-col my-auto">
                    <div class="bg-surface-container-lowest p-8 md:p-12 relative flex-1 overflow-y-auto">
                        
                        <button on:click=move |_| close_modal() class="absolute top-8 right-8 text-outline hover:text-error transition-colors">
                            <span class="material-symbols-outlined text-3xl">"close"</span>
                        </button>
                        
                        <div class="mb-12 border-b-2 border-outline-variant/30 pb-6">
                            <h2 class="text-3xl font-extrabold text-primary uppercase tracking-widest">
                                {move || match modal_state.get() {
                                    ModalState::Job(None) => "NEW JOB",
                                    ModalState::Job(Some(_)) => "EDIT JOB",
                                    ModalState::Project(None) => "NEW PROJECT",
                                    ModalState::Project(Some(_)) => "EDIT PROJECT",
                                    ModalState::Cert(None) => "NEW CERTIFICATION",
                                    ModalState::Cert(Some(_)) => "EDIT CERTIFICATION",
                                    ModalState::Post(None) => "NEW BLOG POST",
                                    ModalState::Post(Some(_)) => "EDIT BLOG POST",
                                    ModalState::Profile(None) => "NEW RESUME PROFILE",
                                    ModalState::Profile(Some(_)) => "EDIT RESUME PROFILE",
                                    ModalState::Passkey => "REGISTER NEW PASSKEY",
                                    ModalState::Settings => "EDIT SITE SETTINGS",
                                    ModalState::None => "",
                                }}
                            </h2>
                        </div>

                        // Modal Form Content
                        <div class="space-y-8">
                            {move || match modal_state.get() {
                                ModalState::Job(j) => view! { <JobForm initial_job=j /> }.into_view(),
                                ModalState::Project(p) => view! { <ProjectForm initial_project=p /> }.into_view(),
                                ModalState::Cert(c) => view! { <CertForm initial_cert=c /> }.into_view(),
                                ModalState::Post(p) => view! { <PostForm initial_post=p /> }.into_view(),
                                ModalState::Profile(p) => view! { <ResumeProfileForm initial_profile=p /> }.into_view(),
                                ModalState::Passkey => view! { <PasskeyForm /> }.into_view(),
                                ModalState::Settings => view! { <SettingsForm /> }.into_view(),
                                ModalState::None => view! { <div/> }.into_view(),
                            }}
                        </div>

                    </div>
                </div>
            </div>
        </Show>
    }
}

// -----------------------------------------
// Job Form
// -----------------------------------------
#[component]
pub fn JobForm(initial_job: Option<crate::pages::resume::JobRecord>) -> impl IntoView {
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let is_edit = initial_job.is_some();
    let id_val = initial_job.as_ref().map(|j| j.id).unwrap_or(0);

    let (date_range, set_date_range) = create_signal(initial_job.as_ref().map(|j| j.date_range.clone()).unwrap_or_default());
    let (role, set_role) = create_signal(initial_job.as_ref().map(|j| j.role.clone()).unwrap_or_default());
    let (company, set_company) = create_signal(initial_job.as_ref().map(|j| j.company.clone()).unwrap_or_default());
    let (bullets, set_bullets) = create_signal(initial_job.as_ref().map(|j| j.bullets.join("\n")).unwrap_or_default());
    let (is_client, set_is_client) = create_signal(initial_job.as_ref().map(|j| j.is_client_project).unwrap_or(true));
    let (tags, set_tags) = create_signal(initial_job.as_ref().map(|j| j.tags.join(", ")).unwrap_or_default());
    let (hide_date, set_hide_date) = create_signal(initial_job.as_ref().map(|j| j.hide_date).unwrap_or(false));

    let save = move |_| {
        let dr = date_range.get_untracked();
        let r = role.get_untracked();
        let c = company.get_untracked();
        let b: Vec<String> = bullets.get_untracked().split('\n').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        let ic = is_client.get_untracked();
        let tg: Vec<String> = tags.get_untracked().split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
        let hd = hide_date.get_untracked();

        spawn_local(async move {
            if is_edit {
                let _ = crate::pages::resume::update_job(id_val, dr, r, c, b, ic, tg, hd).await;
            } else {
                let _ = crate::pages::resume::add_job(dr, r, c, b, ic, tg, hd).await;
            }
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Date Range"</label>
                    <input type="text" prop:value=date_range on:input=move |ev| set_date_range.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Company"</label>
                    <input type="text" prop:value=company on:input=move |ev| set_company.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Role"</label>
                <input type="text" prop:value=role on:input=move |ev| set_role.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Tags (CSV)"</label>
                <input type="text" prop:value=tags on:input=move |ev| set_tags.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Bullets (1 per line)"</label>
                <textarea prop:value=bullets on:input=move |ev| set_bullets.set(event_target_value(&ev)) rows="5" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
            </div>
            <div class="flex items-center gap-3">
                <input type="checkbox" prop:checked=is_client on:change=move |ev| set_is_client.set(event_target_checked(&ev)) class="w-4 h-4 text-primary bg-surface border-outline-variant focus:ring-primary focus:ring-2" />
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Is Client Project (Displays on upper index)"</label>
            </div>
            <div class="flex items-center gap-3">
                <input type="checkbox" prop:checked=hide_date on:change=move |ev| set_hide_date.set(event_target_checked(&ev)) class="w-4 h-4 text-primary bg-surface border-outline-variant focus:ring-primary focus:ring-2" />
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Hide Date Option (Resume Builder)"</label>
            </div>
            <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                "COMMIT TO DATABASE"
            </button>
        </div>
    }
}

// -----------------------------------------
// Project Form
// -----------------------------------------
#[component]
pub fn ProjectForm(initial_project: Option<crate::pages::projects::ProjectRecord>) -> impl IntoView {
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let is_edit = initial_project.is_some();
    let id_val = initial_project.as_ref().map(|p| p.id).unwrap_or(0);

    let (title, set_title) = create_signal(initial_project.as_ref().map(|p| p.title.clone()).unwrap_or_default());
    let (slug, set_slug) = create_signal(initial_project.as_ref().map(|p| p.slug.clone()).unwrap_or_default());
    let (impact, set_impact) = create_signal(initial_project.as_ref().map(|p| p.impact.clone()).unwrap_or_default());
    let (tags, set_tags) = create_signal(initial_project.as_ref().map(|p| p.tags.join(", ")).unwrap_or_default());
    let (bullets, set_bullets) = create_signal(initial_project.as_ref().map(|p| p.bullets.join("\n")).unwrap_or_default());
    let (status, set_status) = create_signal(initial_project.as_ref().map(|p| p.status.clone()).unwrap_or_else(|| "IN PROGRESS".to_string()));
    let (date_range, set_date_range) = create_signal(initial_project.as_ref().map(|p| p.date_range.clone()).unwrap_or_default());

    let save = move |_| {
        let t = title.get_untracked();
        let s = slug.get_untracked();
        let i = impact.get_untracked();
        let st = status.get_untracked();
        let tg: Vec<String> = tags.get_untracked().split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
        let b: Vec<String> = bullets.get_untracked().split('\n').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
        let dr = date_range.get_untracked();

        spawn_local(async move {
            if is_edit {
                let _ = crate::pages::projects::update_project(id_val, t, s, i, tg, b, st, dr).await;
            } else {
                let _ = crate::pages::projects::add_project(t, s, i, tg, b, st, dr).await;
            }
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Title"</label>
                    <input type="text" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Repo Slug"</label>
                    <input type="text" prop:value=slug on:input=move |ev| set_slug.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Impact"</label>
                <input type="text" prop:value=impact on:input=move |ev| set_impact.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Date Range"</label>
                <input type="text" prop:value=date_range on:input=move |ev| set_date_range.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains placeholder:text-outline-variant" placeholder="e.g. Q4 2026" />
            </div>
            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Tags (CSV)"</label>
                    <input type="text" prop:value=tags on:input=move |ev| set_tags.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Project Status"</label>
                    <select on:change=move |ev| set_status.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains text-on-surface">
                        <option value="IN PROGRESS" selected=status.get_untracked() == "IN PROGRESS">"IN PROGRESS"</option>
                        <option value="COMPLETED" selected=status.get_untracked() == "COMPLETED">"COMPLETED"</option>
                    </select>
                </div>
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Bullets (1 per line)"</label>
                <textarea prop:value=bullets on:input=move |ev| set_bullets.set(event_target_value(&ev)) rows="5" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
            </div>
            <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                "COMMIT TO DATABASE"
            </button>
        </div>
    }
}

// -----------------------------------------
// Cert Form
// -----------------------------------------
#[component]
pub fn CertForm(initial_cert: Option<crate::pages::certifications::CertRecord>) -> impl IntoView {
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let is_edit = initial_cert.is_some();
    let id_val = initial_cert.as_ref().map(|c| c.id).unwrap_or(0);

    let (title, set_title) = create_signal(initial_cert.as_ref().map(|c| c.title.clone()).unwrap_or_default());
    let (date_range, set_date_range) = create_signal(initial_cert.as_ref().map(|c| c.date_range.clone()).unwrap_or_default());
    let (is_training, set_is_training) = create_signal(initial_cert.as_ref().map(|c| c.is_training).unwrap_or(false));

    let save = move |_| {
        let t = title.get_untracked();
        let dr = date_range.get_untracked();
        let it = is_training.get_untracked();

        spawn_local(async move {
            if is_edit {
                let _ = crate::pages::certifications::update_certification(id_val, dr, t, it).await;
            } else {
                let _ = crate::pages::certifications::add_certification(dr, t, it).await;
            }
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Certification Title"</label>
                <input type="text" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Date Earned"</label>
                <input type="text" prop:value=date_range on:input=move |ev| set_date_range.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains placeholder:text-outline-variant" placeholder="e.g. 2026.03.14" />
            </div>
            <div class="flex items-center gap-3 mt-4">
                <input type="checkbox" prop:checked=is_training on:change=move |ev| set_is_training.set(event_target_checked(&ev)) class="w-4 h-4 text-primary bg-surface border-outline-variant focus:ring-primary focus:ring-2" />
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Is Training (Vs Accredited Certification)"</label>
            </div>
            <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                "COMMIT TO DATABASE"
            </button>
        </div>
    }
}

// -----------------------------------------
// Post Form (Markdown)
// -----------------------------------------
#[component]
pub fn PostForm(initial_post: Option<crate::pages::blog::PostRecord>) -> impl IntoView {
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let is_edit = initial_post.is_some();
    let id_val = initial_post.as_ref().map(|p| p.id).unwrap_or(0);

    let (title, set_title) = create_signal(initial_post.as_ref().map(|p| p.title.clone()).unwrap_or_default());
    let (slug, set_slug) = create_signal(initial_post.as_ref().map(|p| p.slug.clone()).unwrap_or_default());
    let (tags, set_tags) = create_signal(initial_post.as_ref().map(|p| p.tags.join(", ")).unwrap_or_default());
    let (content, set_content) = create_signal(initial_post.as_ref().map(|p| p.content.clone()).unwrap_or_default());

    let save = move |_| {
        let t = title.get_untracked();
        let s = slug.get_untracked();
        let c = content.get_untracked();
        let tg: Vec<String> = tags.get_untracked().split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();

        spawn_local(async move {
            if is_edit {
                let _ = crate::pages::blog::update_post(id_val, s, t, c, tg).await;
            } else {
                let _ = crate::pages::blog::add_post(s, t, c, tg).await;
            }
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Title"</label>
                    <input type="text" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Slug (URL)"</label>
                    <input type="text" prop:value=slug on:input=move |ev| set_slug.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Tags (CSV)"</label>
                <input type="text" prop:value=tags on:input=move |ev| set_tags.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
            </div>
            <div class="flex flex-col gap-2 mt-4">
                <div class="flex justify-between items-end mb-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Markdown Content"</label>
                    <span class="jetbrains text-[0.55rem] text-secondary tracking-widest">"PULLDOWN-CMARK // ACTIVE"</span>
                </div>
                <textarea prop:value=content on:input=move |ev| set_content.set(event_target_value(&ev)) rows="15" class="bg-surface p-4 border border-outline-variant focus:border-primary focus:ring-0 text-sm font-mono text-on-surface resize-y whitespace-pre block w-full"></textarea>
            </div>
            <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                "COMMIT TO DATABASE"
            </button>
        </div>
    }
}

// -----------------------------------------
// Passkey Form (New Device Binding)
// -----------------------------------------
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/public/webauthn.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn registerDevice(optionsJson: &str) -> Result<JsValue, JsValue>;
}

#[component]
pub fn PasskeyForm() -> impl IntoView {
    #[allow(unused_variables)]
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    #[allow(unused_variables)]
    let set_refresh = expect_context::<WriteSignal<i32>>();
    #[allow(unused_variables)]
    let refresh = expect_context::<ReadSignal<i32>>();

    let (username, set_username) = create_signal(String::new());

    let save = move |_| {
        let uname = username.get_untracked();
        if uname.is_empty() { return; }

        spawn_local(async move {
            if let Ok(_payload) = crate::auth::register_start(uname.clone()).await {
                #[cfg(target_arch = "wasm32")]
                {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&_payload) {
                        if let (Some(c_str), Some(o_str)) = (val["challenge_id"].as_str(), val["options"].as_str()) {
                            if let Ok(challenge_id) = uuid::Uuid::parse_str(c_str) {
                                if let Ok(cred_js) = registerDevice(o_str).await {
                                    if let Some(cred_str) = cred_js.as_string() {
                                        if let Ok(_) = crate::auth::register_finish(uname, challenge_id, cred_str).await {
                                            set_refresh.set(refresh.get_untracked() + 1);
                                            set_modal_state.set(ModalState::None);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    };

    view! {
        <div class="space-y-6">
            <div class="bg-secondary-container/10 p-4 border border-secondary mb-6">
                <p class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider leading-relaxed">
                    "WebAuthn Passkeys register Native Device Keys (Secure Enclave, YubiKey) against a unique Identity Hash. Enter a new identity name below to trigger the system challenge."
                </p>
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Identity Hash / Target Username"</label>
                <input type="text" prop:value=username on:input=move |ev| set_username.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="ex. admin_ipad" />
            </div>
            <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                "TRIGGER HARDWARE CHALLENGE"
            </button>
        </div>
    }
}

// -----------------------------------------
// Settings Form
// -----------------------------------------
#[component]
pub fn SettingsForm() -> impl IntoView {
    use crate::pages::landing::get_site_settings;
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    
    let settings_res = create_resource(|| (), |_| get_site_settings());
    
    let (current_focus, set_current_focus) = create_signal(String::new());
    let (status, set_status) = create_signal(String::new());
    let (hero_quote, set_hero_quote) = create_signal(String::new());
    let (hero_subtitle, set_hero_subtitle) = create_signal(String::new());
    let (site_title, set_site_title) = create_signal(String::new());
    let (lc_title, set_lc_title) = create_signal(String::new());
    let (lc_desc, set_lc_desc) = create_signal(String::new());
    let (lc_label, set_lc_label) = create_signal(String::new());
    let (lc_placeholder, set_lc_placeholder) = create_signal(String::new());
    let (lc_btn, set_lc_btn) = create_signal(String::new());
    let (lc_footer, set_lc_footer) = create_signal(String::new());
    let (lc_endpoint, set_lc_endpoint) = create_signal(String::new());
    let (status_color, set_status_color) = create_signal(String::new());
    let (webhook_url, set_webhook_url) = create_signal(String::new());
    let (admin_email, set_admin_email) = create_signal(String::new());
    let (landing_options_json, set_landing_options_json) = create_signal(String::new());
    let (real_estate_title, set_real_estate_title) = create_signal(String::new());
    let (real_estate_desc, set_real_estate_desc) = create_signal(String::new());
    let (re_lc_title, set_re_lc_title) = create_signal(String::new());
    let (re_lc_desc, set_re_lc_desc) = create_signal(String::new());
    let (re_lc_label, set_re_lc_label) = create_signal(String::new());
    let (re_lc_placeholder, set_re_lc_placeholder) = create_signal(String::new());
    let (re_lc_btn, set_re_lc_btn) = create_signal(String::new());
    let (re_options_json, set_re_options_json) = create_signal(String::new());

    create_effect(move |_| {
        if let Some(Ok(s)) = settings_res.get() {
            set_current_focus.set(s.current_focus);
            set_status.set(s.status);
            set_hero_quote.set(s.hero_quote);
            set_hero_subtitle.set(s.hero_subtitle);
            set_site_title.set(s.site_title);
            set_lc_title.set(s.lc_title);
            set_lc_desc.set(s.lc_desc);
            set_lc_label.set(s.lc_label);
            set_lc_placeholder.set(s.lc_placeholder);
            set_lc_btn.set(s.lc_btn);
            set_lc_footer.set(s.lc_footer);
            set_lc_endpoint.set(s.lc_endpoint);
            set_status_color.set(s.status_color);
            set_webhook_url.set(s.webhook_url);
            set_admin_email.set(s.admin_email);
            set_landing_options_json.set(s.landing_options_json);
            set_real_estate_title.set(s.real_estate_title);
            set_real_estate_desc.set(s.real_estate_desc);
            set_re_lc_title.set(s.re_lc_title);
            set_re_lc_desc.set(s.re_lc_desc);
            set_re_lc_label.set(s.re_lc_label);
            set_re_lc_placeholder.set(s.re_lc_placeholder);
            set_re_lc_btn.set(s.re_lc_btn);
            set_re_options_json.set(s.re_options_json);
        }
    });

    let save = move |_| {
        let cf = current_focus.get_untracked();
        let st = status.get_untracked();
        let hq = hero_quote.get_untracked();
        let hs = hero_subtitle.get_untracked();
        let sttl = site_title.get_untracked();
        let lt = lc_title.get_untracked();
        let ld = lc_desc.get_untracked();
        let ll = lc_label.get_untracked();
        let lp = lc_placeholder.get_untracked();
        let lb = lc_btn.get_untracked();
        let lf = lc_footer.get_untracked();
        let le = lc_endpoint.get_untracked();
        let sc = status_color.get_untracked();
        let wu = webhook_url.get_untracked();
        let ae = admin_email.get_untracked();
        let loj = landing_options_json.get_untracked();
        let ret = real_estate_title.get_untracked();
        let red = real_estate_desc.get_untracked();
        let rlt = re_lc_title.get_untracked();
        let rld = re_lc_desc.get_untracked();
        let rll = re_lc_label.get_untracked();
        let rlp = re_lc_placeholder.get_untracked();
        let rlb = re_lc_btn.get_untracked();
        let roj = re_options_json.get_untracked();

        spawn_local(async move {
            let _ = crate::pages::landing::update_site_settings(cf, st, hq, hs, sttl, lt, ld, ll, lp, lb, lf, le, sc, wu, ae, loj, ret, red, rlt, rld, rll, rlp, rlb, roj).await;
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <Suspense fallback=move || view! { <div class="jetbrains text-sm">"Hydrating..."</div> }>
            <div class="space-y-6">
                <div class="bg-primary/10 border-l-4 border-primary p-4 mb-8">
                    <p class="jetbrains text-xs text-on-surface uppercase tracking-widest leading-relaxed">
                        "These key-value parameters are injected directly into the hero layout on the Root Navigation landing page."
                    </p>
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Hero Parameter // Current Focus"</label>
                    <input type="text" prop:value=current_focus on:input=move |ev| set_current_focus.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Hero Parameter // Status"</label>
                    <input type="text" prop:value=status on:input=move |ev| set_status.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Hero Parameter // Subtitle"</label>
                    <textarea prop:value=hero_subtitle on:input=move |ev| set_hero_subtitle.set(event_target_value(&ev)) rows="3" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Hero Parameter // Quote"</label>
                    <textarea prop:value=hero_quote on:input=move |ev| set_hero_quote.set(event_target_value(&ev)) rows="3" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider mt-4">"Lead Capture Parameter // Site Title"</label>
                    <input type="text" prop:value=site_title on:input=move |ev| set_site_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider mt-4">"Real Estate Parameter // Title"</label>
                    <input type="text" prop:value=real_estate_title on:input=move |ev| set_real_estate_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider mt-4">"Real Estate Parameter // Description"</label>
                    <textarea prop:value=real_estate_desc on:input=move |ev| set_real_estate_desc.set(event_target_value(&ev)) rows="3" class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains resize-y"></textarea>
                </div>
                <div class="grid grid-cols-2 gap-4">
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider">"Lead Capture Parameter // Title"</label>
                        <input type="text" prop:value=lc_title on:input=move |ev| set_lc_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" />
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider">"Lead Capture Parameter // Description"</label>
                        <input type="text" prop:value=lc_desc on:input=move |ev| set_lc_desc.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" />
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-4">
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider">"Lead Capture Parameter // Form Label"</label>
                        <input type="text" prop:value=lc_label on:input=move |ev| set_lc_label.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" />
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider">"Lead Capture Parameter // Placeholder"</label>
                        <input type="text" prop:value=lc_placeholder on:input=move |ev| set_lc_placeholder.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" />
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-4">
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider">"Lead Capture Parameter // Button Stack"</label>
                        <input type="text" prop:value=lc_btn on:input=move |ev| set_lc_btn.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" />
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider">"Lead Capture Parameter // Endpoint Target"</label>
                        <input type="text" prop:value=lc_endpoint on:input=move |ev| set_lc_endpoint.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains font-mono text-secondary" />
                    </div>
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider">"Lead Capture Parameter // Footer Disclaimer"</label>
                    <input type="text" prop:value=lc_footer on:input=move |ev| set_lc_footer.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Hero Parameter // Status LED Color"</label>
                    <div class="flex items-center gap-4">
                        <select on:change=move |ev| {
                                let v = event_target_value(&ev);
                                if v != "custom" { set_status_color.set(v); }
                            } 
                            class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains font-mono uppercase"
                        >
                            <option value="custom">"Custom Color..."</option>
                            <option value="#ff5449" selected=move || status_color.get() == "#ff5449">"Error (Red) [#FF5449]"</option>
                            <option value="#34d399" selected=move || status_color.get() == "#34d399">"Success (Green) [#34D399]"</option>
                            <option value="#fbfaf8" selected=move || status_color.get() == "#fbfaf8">"Inactive (White) [#FBFAF8]"</option>
                            <option value="#f7931a" selected=move || status_color.get() == "#f7931a">"Bitcoin (Orange) [#F7931A]"</option>
                            <option value="#22d3ee" selected=move || status_color.get() == "#22d3ee">"Primary (Cyan) [#22D3EE]"</option>
                        </select>
                        <input type="color" prop:value=status_color on:input=move |ev| set_status_color.set(event_target_value(&ev)) class="w-12 h-12 bg-surface border border-outline-variant cursor-pointer p-1" />
                        <input type="text" prop:value=status_color on:input=move |ev| set_status_color.set(event_target_value(&ev)) class="bg-surface flex-1 p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains font-mono uppercase" placeholder="#FFFFFF" />
                    </div>
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Webhook URL (On Lead Capture)"</label>
                    <input type="text" prop:value=webhook_url on:input=move |ev| set_webhook_url.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="https://..." />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Admin Notification Email"</label>
                    <input type="text" prop:value=admin_email on:input=move |ev| set_admin_email.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="admin@domain.com" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Landing Options (JSON Checkboxes)"</label>
                    <textarea prop:value=landing_options_json on:input=move |ev| set_landing_options_json.set(event_target_value(&ev)) rows="3" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains font-mono resize-y" placeholder="{{ \"option_id\": \"Option Label\" }}"></textarea>
                </div>
                
                <div class="pt-6 border-t border-outline-variant/30 space-y-6">
                    <h3 class="font-label text-sm font-bold text-primary tracking-widest uppercase">"Real Estate Lead Capture Configuration"</h3>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="flex flex-col gap-2">
                            <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"LC Title"</label>
                            <input type="text" prop:value=re_lc_title on:input=move |ev| set_re_lc_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                        </div>
                        <div class="flex flex-col gap-2">
                            <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"LC Input Label"</label>
                            <input type="text" prop:value=re_lc_label on:input=move |ev| set_re_lc_label.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                        </div>
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"LC Description"</label>
                        <textarea prop:value=re_lc_desc on:input=move |ev| set_re_lc_desc.set(event_target_value(&ev)) rows="2" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="flex flex-col gap-2">
                            <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"LC Input Placeholder"</label>
                            <input type="text" prop:value=re_lc_placeholder on:input=move |ev| set_re_lc_placeholder.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                        </div>
                        <div class="flex flex-col gap-2">
                            <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"LC Button Text"</label>
                            <input type="text" prop:value=re_lc_btn on:input=move |ev| set_re_lc_btn.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                        </div>
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Checkboxes (JSON Map: value => label)"</label>
                        <textarea prop:value=re_options_json on:input=move |ev| set_re_options_json.set(event_target_value(&ev)) rows="3" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains font-mono resize-y text-secondary text-xs"></textarea>
                    </div>
                </div>

                <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                    "OVERWRITE GLOBAL SETTINGS"
                </button>
            </div>
        </Suspense>
    }
}

// -----------------------------------------
// Resume Profile Form
// -----------------------------------------
#[component]
pub fn ResumeProfileForm(initial_profile: Option<crate::resume_engine::ResumeProfile>) -> impl IntoView {
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let is_edit = initial_profile.is_some();
    let id_val = initial_profile.as_ref().map(|p| p.id).unwrap_or(0);

    let (name, set_name) = create_signal(initial_profile.as_ref().map(|p| p.name.clone()).unwrap_or_default());
    let (biography, set_biography) = create_signal(initial_profile.as_ref().map(|p| p.biography.clone()).unwrap_or_else(|| "Specializing in Enterprise Cloud Solutions, APEX, and Rust External Microservices. Dedicated to translating complex systems into immutable data flows.".to_string()));
    let (excluded_tags, set_excluded_tags) = create_signal(initial_profile.as_ref().map(|p| p.excluded_tags.join(", ")).unwrap_or_default());
    let (anon_json, set_anon_json) = create_signal(initial_profile.as_ref().map(|p| serde_json::to_string(&p.anonymous_companies).unwrap_or_else(|_| "{}".to_string())).unwrap_or_else(|| "{}".to_string()));

    let save = move |_| {
        let n = name.get_untracked();
        let b = biography.get_untracked();
        let et: Vec<String> = excluded_tags.get_untracked().split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
        let aj = anon_json.get_untracked();
        let anon: std::collections::HashMap<String, String> = serde_json::from_str(&aj).unwrap_or_default();

        spawn_local(async move {
            if is_edit {
                let _ = crate::resume_engine::update_resume_profile(id_val, n, b, et, anon).await;
            } else {
                let _ = crate::resume_engine::add_resume_profile(n, b, et, anon).await;
            }
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Profile Name"</label>
                <input type="text" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="e.g. CyberSecurity Spec" />
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Custom Biography"</label>
                <textarea prop:value=biography on:input=move |ev| set_biography.set(event_target_value(&ev)) rows="5" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Excluded Tags (CSV)"</label>
                <input type="text" prop:value=excluded_tags on:input=move |ev| set_excluded_tags.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="e.g. rust, legacy" />
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Anonymous Companies (JSON Map)"</label>
                <textarea prop:value=anon_json on:input=move |ev| set_anon_json.set(event_target_value(&ev)) rows="3" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains font-mono resize-y text-secondary" placeholder="{{ \"Acme Corp\": \"Confidential FinTech Client\" }}"></textarea>
            </div>
            <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                "COMMIT TO DATABASE"
            </button>
        </div>
    }
}
