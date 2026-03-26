use leptos::*;

#[derive(Clone, PartialEq, Debug)]
pub enum ModalState {
    None,
    Job(Option<crate::pages::resume::JobRecord>),
    Project(Option<crate::pages::projects::ProjectRecord>),
    Cert(Option<crate::pages::certifications::CertRecord>),
    Post(Option<crate::pages::blog::PostRecord>),
    Profile(Option<crate::resume_engine::ResumeProfile>),
    LandingPage(Option<crate::pages::dynamic_landing::LandingPageRecord>),
    MailingList(Option<crate::pages::admin::MailingListRecord>),
    NavItem(Option<crate::components::nav::NavItemRecord>),
    FooterItem(Option<crate::components::footer::FooterItemRecord>),
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
                    <button on:click=move |_| close_modal() class="absolute -top-4 -right-4 p-3 z-50 bg-surface-container-high border border-outline-variant/30 rounded-full text-outline hover:text-error hover:border-error transition-all shadow-xl">
                        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                    <div class="bg-surface-container-lowest p-8 md:p-12 relative flex-1 overflow-y-auto">
                        
                        <div class="mb-12 border-b-2 border-outline-variant/30 pb-6 mt-4">
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
                                    ModalState::LandingPage(None) => "NEW LANDING PAGE",
                                    ModalState::LandingPage(Some(_)) => "EDIT LANDING PAGE",
                                    ModalState::MailingList(None) => "ADD MAILING LIST MEMBER",
                                    ModalState::MailingList(Some(_)) => "EDIT MAILING LIST MEMBER",
                                    ModalState::NavItem(None) => "NEW NAVIGATION NODE",
                                    ModalState::NavItem(Some(_)) => "EDIT NAVIGATION NODE",
                                    ModalState::FooterItem(None) => "NEW FOOTER NODE",
                                    ModalState::FooterItem(Some(_)) => "EDIT FOOTER NODE",
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
                                ModalState::LandingPage(p) => view! { <LandingPageForm initial_page=p /> }.into_view(),
                                ModalState::MailingList(r) => view! { <MailingListForm initial_record=r /> }.into_view(),
                                ModalState::NavItem(n) => view! { <NavItemForm initial_item=n /> }.into_view(),
                                ModalState::FooterItem(f) => view! { <FooterItemForm initial_item=f /> }.into_view(),
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
    let (employment_type, set_employment_type) = create_signal(initial_job.as_ref().map(|j| j.employment_type.clone()).unwrap_or_default());
    let (parent_company, set_parent_company) = create_signal(initial_job.as_ref().and_then(|j| j.parent_company.clone()).unwrap_or_default());
    let (tags, set_tags) = create_signal(initial_job.as_ref().map(|j| j.tags.join(", ")).unwrap_or_default());
    let (hide_date, set_hide_date) = create_signal(initial_job.as_ref().map(|j| j.hide_date).unwrap_or(false));

    let save = move |_| {
        let dr = date_range.get_untracked();
        let r = role.get_untracked();
        let c = company.get_untracked();
        let b: Vec<String> = bullets.get_untracked().split('\n').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        let et = employment_type.get_untracked();
        let pc = parent_company.get_untracked();
        let pc_opt = if et == crate::pages::resume::JobType::CorpToCorp && !pc.is_empty() { Some(pc) } else { None };
        let tg: Vec<String> = tags.get_untracked().split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
        let hd = hide_date.get_untracked();

        spawn_local(async move {
            if is_edit {
                let _ = crate::pages::resume::update_job(id_val, dr, r, c, b, et.clone(), pc_opt.clone(), tg, hd).await;
            } else {
                let _ = crate::pages::resume::add_job(dr, r, c, b, et.clone(), pc_opt.clone(), tg, hd).await;
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
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Employment Type"</label>
                <select on:change=move |ev| set_employment_type.set(event_target_value(&ev).parse().unwrap_or_default()) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains outline-none">
                    <option value="DirectHire" selected=move || employment_type.get() == crate::pages::resume::JobType::DirectHire>"Direct Hire (W2)"</option>
                    <option value="Contract" selected=move || employment_type.get() == crate::pages::resume::JobType::Contract>"Contract / 1099"</option>
                    <option value="CorpToCorp" selected=move || employment_type.get() == crate::pages::resume::JobType::CorpToCorp>"Corp-to-Corp (C2C)"</option>
                </select>
            </div>
            
            {move || {
                if employment_type.get() == crate::pages::resume::JobType::CorpToCorp {
                    view! {
                        <div class="flex flex-col gap-2">
                            <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider text-secondary">"Parent Company (C2C Entity)"</label>
                            <input type="text" prop:value=parent_company on:input=move |ev| set_parent_company.set(event_target_value(&ev)) class="bg-surface p-3 border border-secondary/50 focus:border-secondary focus:ring-0 text-sm jetbrains" placeholder="e.g. Oplyst International, LLC" />
                        </div>
                    }.into_view()
                } else {
                    view! { <span class="hidden"></span> }.into_view()
                }
            }}            <div class="flex items-center gap-3">
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
    let (is_loading, set_is_loading) = create_signal(false);
    let (auth_error, set_auth_error) = create_signal(String::new());

    let save = move |_| {
        let uname = username.get_untracked();
        if uname.is_empty() { 
            set_auth_error.set("Identity Hash (Username) required.".to_string());
            return; 
        }

        set_is_loading.set(true);
        set_auth_error.set(String::new());

        spawn_local(async move {
            match crate::auth::register_start(uname.clone()).await {
                Ok(_payload) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&_payload) {
                            if let (Some(c_str), Some(o_str)) = (val["challenge_id"].as_str(), val["options"].as_str()) {
                                if let Ok(challenge_id) = uuid::Uuid::parse_str(c_str) {
                                    match registerDevice(o_str).await {
                                        Ok(cred_js) => {
                                            if let Some(cred_str) = cred_js.as_string() {
                                                match crate::auth::register_finish(uname, challenge_id, cred_str).await {
                                                    Ok(_) => {
                                                        set_refresh.set(refresh.get_untracked() + 1);
                                                        set_modal_state.set(ModalState::None);
                                                    },
                                                    Err(e) => set_auth_error.set(format!("Validation failed: {:?}", e)),
                                                }
                                            } else {
                                                set_auth_error.set("Invalid browser credential.".to_string());
                                            }
                                        },
                                        Err(_) => set_auth_error.set("Device rejected or cancelled.".to_string()),
                                    }
                                } else {
                                    set_auth_error.set("Bad challenge ID".to_string());
                                }
                            } else {
                                set_auth_error.set("Malformed payload".to_string());
                            }
                        } else {
                            set_auth_error.set("JSON parse error".to_string());
                        }
                    }
                },
                Err(e) => set_auth_error.set(format!("Server error: {:?}", e)),
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="bg-secondary-container/10 p-4 border border-secondary mb-6">
                <p class="jetbrains text-[0.65rem] uppercase text-secondary tracking-wider leading-relaxed">
                    "WebAuthn Passkeys register Native Device Keys (Secure Enclave, YubiKey) against a unique Identity Hash. Enter a new identity name below to trigger the system challenge."
                </p>
            </div>
            
            <Show when=move || !auth_error.get().is_empty()>
                <div class="bg-error/10 border-l-4 border-error p-4 text-error jetbrains text-sm font-medium">
                    {move || auth_error.get()}
                </div>
            </Show>

            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Identity Hash / Target Username"</label>
                <input type="text" prop:value=username on:input=move |ev| set_username.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="ex. admin_ipad" />
            </div>
            <button 
                on:click=save 
                disabled=is_loading
                class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex justify-center items-center gap-3"
            >
                <Show when=move || is_loading.get()>
                    <span class="material-symbols-outlined animate-spin text-base">"progress_activity"</span>
                </Show>
                <span class="inline-block translate-y-[1px]">"TRIGGER HARDWARE CHALLENGE"</span>
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
    let (objective, set_objective) = create_signal(initial_profile.as_ref().and_then(|p| p.objective.clone()).unwrap_or_default());
    let (is_public, set_is_public) = create_signal(initial_profile.as_ref().map(|p| p.is_public).unwrap_or(false));
    
    let (target_role, set_target_role) = create_signal(initial_profile.as_ref().and_then(|p| p.target_role.clone()).unwrap_or_default());
    let (contact_email, set_contact_email) = create_signal(initial_profile.as_ref().and_then(|p| p.contact_email.clone()).unwrap_or_default());
    let (contact_phone, set_contact_phone) = create_signal(initial_profile.as_ref().and_then(|p| p.contact_phone.clone()).unwrap_or_default());
    let (contact_location, set_contact_location) = create_signal(initial_profile.as_ref().and_then(|p| p.contact_location.clone()).unwrap_or_default());
    let (contact_link, set_contact_link) = create_signal(initial_profile.as_ref().and_then(|p| p.contact_link.clone()).unwrap_or_default());

    let (cat_vis_str, set_cat_vis_str) = create_signal(
        initial_profile.as_ref()
            .map(|p| p.category_visibility.to_string())
            .unwrap_or_else(|| "{\n  \"work\": true,\n  \"education\": true\n}".to_string())
    );

    let placeholder_text = "{\n  \"work\": true,\n  \"education\": false\n}";

    let save = move |_| {
        let n = name.get_untracked();
        let obj = if objective.get_untracked().is_empty() { None } else { Some(objective.get_untracked()) };
        let p_pub = is_public.get_untracked();
        
        let tr = if target_role.get_untracked().is_empty() { None } else { Some(target_role.get_untracked()) };
        let ce = if contact_email.get_untracked().is_empty() { None } else { Some(contact_email.get_untracked()) };
        let cp = if contact_phone.get_untracked().is_empty() { None } else { Some(contact_phone.get_untracked()) };
        let clo = if contact_location.get_untracked().is_empty() { None } else { Some(contact_location.get_untracked()) };
        let cli = if contact_link.get_untracked().is_empty() { None } else { Some(contact_link.get_untracked()) };
        
        let cv: serde_json::Value = serde_json::from_str(&cat_vis_str.get_untracked()).unwrap_or_else(|_| serde_json::json!({}));

        spawn_local(async move {
            if is_edit {
                let _ = crate::resume_engine::update_resume_profile(id_val, n, obj, p_pub, tr, ce, cp, clo, cli, cv).await;
            } else {
                let _ = crate::resume_engine::add_resume_profile(n, obj, p_pub, tr, ce, cp, clo, cli, cv).await;
            }
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="flex items-center gap-3 bg-surface-container-high p-4 border border-outline-variant/30">
                <input 
                    type="checkbox" 
                    prop:checked=is_public 
                    on:change=move |ev| set_is_public.set(event_target_checked(&ev)) 
                    class="w-5 h-5 text-primary bg-surface border-outline-variant focus:ring-primary focus:ring-2" 
                />
                <div>
                    <div class="font-bold text-sm text-on-surface uppercase tracking-widest">"Public Profile"</div>
                    <div class="text-xs text-outline leading-tight">"If enabled, this profile will be available to visitors on the frontend /resume component."</div>
                </div>
            </div>

            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Profile Name (Internal)"</label>
                    <input type="text" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="e.g. Architect Profile" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Target Role Headline"</label>
                    <input type="text" prop:value=target_role on:input=move |ev| set_target_role.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="e.g. Cloud Engineer" />
                </div>
            </div>

            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Executive Objective / Summary"</label>
                <textarea prop:value=objective on:input=move |ev| set_objective.set(event_target_value(&ev)) rows="3" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
            </div>

            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Contact Email"</label>
                    <input type="email" prop:value=contact_email on:input=move |ev| set_contact_email.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Contact Phone"</label>
                    <input type="text" prop:value=contact_phone on:input=move |ev| set_contact_phone.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
            </div>

            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Location (City, State)"</label>
                    <input type="text" prop:value=contact_location on:input=move |ev| set_contact_location.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"LinkedIn / Website"</label>
                    <input type="text" prop:value=contact_link on:input=move |ev| set_contact_link.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
            </div>

            <div class="flex flex-col gap-2 border-t border-outline-variant/30 pt-6">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Category Visibility Configuration JSON (Enum Keys: work, education, skill, project, language, volunteer, extracurricular, hobby)"</label>
                <textarea prop:value=cat_vis_str on:input=move |ev| set_cat_vis_str.set(event_target_value(&ev)) rows="3" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains font-mono resize-y text-secondary text-xs" placeholder=placeholder_text></textarea>
            </div>

            <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                "COMMIT PROFILE TO DATABASE"
            </button>
        </div>
    }
}

// -----------------------------------------
// Landing Page Form
// -----------------------------------------
#[component]
pub fn LandingPageForm(initial_page: Option<crate::pages::dynamic_landing::LandingPageRecord>) -> impl IntoView {
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let is_edit = initial_page.is_some();
    let id_val = initial_page.as_ref().map(|p| p.id).unwrap_or(0);

    let (slug, set_slug) = create_signal(initial_page.as_ref().map(|p| p.slug.clone()).unwrap_or_default());
    let (title, set_title) = create_signal(initial_page.as_ref().map(|p| p.title.clone()).unwrap_or_default());
    let (description, set_description) = create_signal(initial_page.as_ref().map(|p| p.description.clone()).unwrap_or_default());
    let (hero_title, set_hero_title) = create_signal(initial_page.as_ref().map(|p| p.hero_title.clone()).unwrap_or_default());
    let (hero_subtitle, set_hero_subtitle) = create_signal(initial_page.as_ref().map(|p| p.hero_subtitle.clone()).unwrap_or_default());
    let (lc_title, set_lc_title) = create_signal(initial_page.as_ref().map(|p| p.lead_capture_title.clone()).unwrap_or_default());
    let (lc_desc, set_lc_desc) = create_signal(initial_page.as_ref().map(|p| p.lead_capture_desc.clone()).unwrap_or_default());
    let (lc_btn, set_lc_btn) = create_signal(initial_page.as_ref().map(|p| p.lead_capture_btn.clone()).unwrap_or_default());
    let (options_json, set_options_json) = create_signal(initial_page.as_ref().map(|p| p.options_json.clone()).unwrap_or_else(|| "{\n  \"opt1\": \"First Option\",\n  \"opt2\": \"Second Option\"\n}".to_string()));

    let save = move |_| {
        let s = slug.get_untracked();
        let t = title.get_untracked();
        let d = description.get_untracked();
        let ht = hero_title.get_untracked();
        let hs = hero_subtitle.get_untracked();
        let lct = lc_title.get_untracked();
        let lcd = lc_desc.get_untracked();
        let lcb = lc_btn.get_untracked();
        let oj = options_json.get_untracked();

        spawn_local(async move {
            if is_edit {
                let _ = crate::pages::dynamic_landing::update_landing_page(id_val, s, t, d, ht, hs, lct, lcd, lcb, oj).await;
            } else {
                let _ = crate::pages::dynamic_landing::add_landing_page(s, t, d, ht, hs, lct, lcd, lcb, oj).await;
            }
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Slug (URL path)"</label>
                    <input type="text" prop:value=slug on:input=move |ev| set_slug.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="e.g. real-estate" />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Page Title (Tab Name)"</label>
                    <input type="text" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                </div>
            </div>
            
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Top Description (Small Text)"</label>
                <textarea prop:value=description on:input=move |ev| set_description.set(event_target_value(&ev)) rows="2" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
            </div>
            
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Hero Main Header (Can include HTML)"</label>
                <textarea prop:value=hero_title on:input=move |ev| set_hero_title.set(event_target_value(&ev)) rows="2" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Hero Subtitle (Underneath header)"</label>
                <textarea prop:value=hero_subtitle on:input=move |ev| set_hero_subtitle.set(event_target_value(&ev)) rows="2" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
            </div>

            <div class="border-t border-outline-variant/30 pt-6 mt-4">
                <h3 class="font-label text-sm font-bold text-primary tracking-widest uppercase mb-4">"Lead Capture Block"</h3>
                <div class="grid grid-cols-2 gap-4">
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Lead Capture Title"</label>
                        <input type="text" prop:value=lc_title on:input=move |ev| set_lc_title.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Submit Button Label"</label>
                        <input type="text" prop:value=lc_btn on:input=move |ev| set_lc_btn.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
                    </div>
                </div>
                <div class="flex flex-col gap-2 mt-4">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Lead Capture Description"</label>
                    <textarea prop:value=lc_desc on:input=move |ev| set_lc_desc.set(event_target_value(&ev)) rows="2" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains resize-y"></textarea>
                </div>
                <div class="flex flex-col gap-2 mt-4">
                    <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Checkboxes config (JSON Map)"</label>
                    <textarea prop:value=options_json on:input=move |ev| set_options_json.set(event_target_value(&ev)) rows="5" class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm font-mono text-secondary resize-y" placeholder="{{ \"val_id\": \"User Facing Label\" }}"></textarea>
                </div>
            </div>

            <button on:click=save class="mt-8 bg-primary text-on-primary font-bold jetbrains uppercase w-full py-4 tracking-widest hover:bg-primary-container transition-colors">
                "COMMIT TO DATABASE"
            </button>
        </div>
    }
}

// -----------------------------------------
// Mailing List Form
// -----------------------------------------
#[component]
pub fn MailingListForm(initial_record: Option<crate::pages::admin::MailingListRecord>) -> impl IntoView {
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let (email, set_email) = create_signal(initial_record.as_ref().map(|r| r.email.clone()).unwrap_or_default());
    let (list_type, set_list_type) = create_signal(initial_record.as_ref().map(|r| r.list_type.clone()).unwrap_or_else(|| "manual_override".to_string()));

    let save = move |_| {
        let e = email.get_untracked();
        let lt = list_type.get_untracked();
        
        spawn_local(async move {
            let _ = crate::pages::dynamic_landing::handle_dynamic_lead(lt, e, vec![]).await;
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"Lead Email"</label>
                <input type="email" prop:value=email on:input=move |ev| set_email.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" placeholder="guest@example.com" />
            </div>
            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider">"List Identifier (Tags)"</label>
                <input type="text" prop:value=list_type on:input=move |ev| set_list_type.set(event_target_value(&ev)) class="bg-surface p-3 border border-outline-variant focus:border-primary focus:ring-0 text-sm jetbrains" />
            </div>
            
            <button on:click=save class="w-full bg-primary text-on-primary py-4 mt-6 jetbrains text-xs font-bold tracking-[0.2em] uppercase hover:bg-primary-container transition-colors shadow-lg">
                "SUBMIT LEAD"
            </button>
        </div>
    }
}

// -----------------------------------------
// Navigation Item Form
// -----------------------------------------
#[component]
pub fn NavItemForm(initial_item: Option<crate::components::nav::NavItemRecord>) -> impl IntoView {
    use crate::components::nav::*;
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let is_edit = initial_item.is_some();
    let id_val = initial_item.as_ref().map(|p| p.id).unwrap_or(0);

    let (label, set_label) = create_signal(initial_item.as_ref().map(|p| p.label.clone()).unwrap_or_default());
    let (href, set_href) = create_signal(initial_item.as_ref().and_then(|p| p.href.clone()).unwrap_or_default());
    let (parent_id_str, set_parent_id_str) = create_signal(initial_item.as_ref().and_then(|p| p.parent_id).map(|id| id.to_string()).unwrap_or_default());
    let (display_order, set_display_order) = create_signal(initial_item.as_ref().map(|p| p.display_order.to_string()).unwrap_or_else(|| "0".to_string()));
    let (is_visible, set_is_visible) = create_signal(initial_item.as_ref().map(|p| p.is_visible).unwrap_or(true));

    let (loading, set_loading) = create_signal(false);
    
    // Quick dropdown loader for parent
    let parents_resource = create_resource(move || refresh.get(), |_| get_all_nav_items());

    let save_action = create_action(move |_: &()| {
        let lbl = label.get_untracked();
        let hr = href.get_untracked();
        let pid_str = parent_id_str.get_untracked();
        let ord_str = display_order.get_untracked();
        let vis = is_visible.get_untracked();
        
        let hr_opt = if hr.is_empty() { None } else { Some(hr) };
        let pid_opt = if pid_str.is_empty() { None } else { pid_str.parse::<i32>().ok() };
        let ord = ord_str.parse::<i32>().unwrap_or(0);

        async move {
            set_loading.set(true);
            if is_edit {
                let _ = update_nav_item(id_val, lbl, hr_opt, pid_opt, ord, vis).await;
            } else {
                let _ = add_nav_item(lbl, hr_opt, pid_opt, ord, vis).await;
            }
            set_loading.set(false);
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        }
    });

    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline">"Display Label"</label>
                    <input 
                        type="text" 
                        prop:value=label 
                        on:input=move |ev| set_label.set(event_target_value(&ev))
                        class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-2 jetbrains text-sm text-on-surface transition-all placeholder:text-outline-variant/30" 
                        placeholder="e.g. BLOG"
                    />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline">"Weight (Order)"</label>
                    <input 
                        type="number" 
                        prop:value=display_order 
                        on:input=move |ev| set_display_order.set(event_target_value(&ev))
                        class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-2 jetbrains text-sm text-on-surface transition-all" 
                    />
                </div>
            </div>

            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline">"Target URL (Href)"</label>
                <input 
                    type="text" 
                    prop:value=href 
                    on:input=move |ev| set_href.set(event_target_value(&ev))
                    class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-2 jetbrains text-sm text-on-surface transition-all placeholder:text-outline-variant/30" 
                    placeholder="e.g. /blog or leave empty for Dropdown Header"
                />
                <span class="text-xs text-outline-variant jetbrains">"If left blank, this item acts as a dropdown menu parent."</span>
            </div>

            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline">"Dropdown Parent Binding"</label>
                <select 
                    on:change=move |ev| set_parent_id_str.set(event_target_value(&ev))
                    class="w-full bg-surface-container-high border-none focus:ring-primary px-4 py-3 jetbrains text-sm text-on-surface outline-none cursor-pointer"
                >
                    <option value="" selected=move || parent_id_str.get().is_empty()>"-- NONE (TOP LEVEL) --"</option>
                    <Suspense fallback=move || view! { <option>"Loading..."</option> }>
                        {move || match parents_resource.get() {
                            Some(Ok(items)) => items.into_iter()
                                .filter(|i| i.href.is_none() || i.href.as_deref() == Some(""))
                                .map(|i| {
                                    let id_s = i.id.to_string();
                                    view! {
                                        <option value=id_s.clone() selected=move || parent_id_str.get() == id_s>
                                            {format!("{} (ID: {})", i.label, i.id)}
                                        </option>
                                    }
                            }).collect_view(),
                            _ => view! { <option disabled=true>"ERROR"</option> }.into_view()
                        }}
                    </Suspense>
                </select>
                <span class="text-xs text-outline-variant jetbrains">"Select a parent item to nest this link inside a dropdown."</span>
            </div>

            <div class="flex items-center gap-3 bg-surface-container/50 p-4 border border-outline-variant/20">
                <label class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" 
                        class="sr-only peer" 
                        prop:checked=is_visible 
                        on:change=move |ev| set_is_visible.set(event_target_checked(&ev))
                    />
                    <div class="w-9 h-5 bg-outline-variant peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-surface after:border-outline-variant after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-primary"></div>
                    <span class="ml-3 text-xs jetbrains font-bold uppercase tracking-widest text-primary">
                        {move || if is_visible.get() { "VISIBLE ON SITE" } else { "HIDDEN ENTIRELY" }}
                    </span>
                </label>
            </div>

            <div class="flex justify-end gap-4 pt-4 border-t-2 border-outline-variant/30">
                <button 
                    on:click=move |_| set_modal_state.set(ModalState::None)
                    class="px-6 py-3 jetbrains text-xs font-bold tracking-widest uppercase text-outline hover:text-on-surface transition-colors"
                    disabled=loading
                >
                    "Cancel"
                </button>
                <button 
                    on:click=move |_| save_action.dispatch(())
                    class="bg-primary text-on-primary px-8 py-3 jetbrains text-xs font-bold tracking-widest uppercase hover:bg-primary-container transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
                    disabled=loading
                >
                    <Show when=move || loading.get()>
                        <span class="material-symbols-outlined animate-spin text-sm">"progress_activity"</span>
                    </Show>
                    {move || if is_edit { "COMMIT CHANGES" } else { "CREATE NODE" }}
                </button>
            </div>
        </div>
    }
}

// -----------------------------------------
// Footer Item Form
// -----------------------------------------
#[component]
pub fn FooterItemForm(initial_item: Option<crate::components::footer::FooterItemRecord>) -> impl IntoView {
    use crate::components::footer::*;
    let set_modal_state = expect_context::<WriteSignal<ModalState>>();
    let set_refresh = expect_context::<WriteSignal<i32>>();
    let refresh = expect_context::<ReadSignal<i32>>();

    let is_edit = initial_item.is_some();
    let id_val = initial_item.as_ref().map(|p| p.id).unwrap_or(0);

    let (label, set_label) = create_signal(initial_item.as_ref().map(|p| p.label.clone()).unwrap_or_default());
    let (href, set_href) = create_signal(initial_item.as_ref().and_then(|p| p.href.clone()).unwrap_or_default());
    let (display_order, set_display_order) = create_signal(initial_item.as_ref().map(|p| p.display_order.to_string()).unwrap_or_else(|| "0".to_string()));
    let (is_visible, set_is_visible) = create_signal(initial_item.as_ref().map(|p| p.is_visible).unwrap_or(true));

    let (loading, set_loading) = create_signal(false);

    let save_action = create_action(move |_: &()| {
        let lbl = label.get_untracked();
        let hr = href.get_untracked();
        let ord_str = display_order.get_untracked();
        let vis = is_visible.get_untracked();
        
        let hr_opt = if hr.is_empty() { None } else { Some(hr) };
        let ord = ord_str.parse::<i32>().unwrap_or(0);

        async move {
            set_loading.set(true);
            if is_edit {
                let _ = update_footer_item(id_val, lbl, hr_opt, ord, vis).await;
            } else {
                let _ = add_footer_item(lbl, hr_opt, ord, vis).await;
            }
            set_loading.set(false);
            set_refresh.set(refresh.get_untracked() + 1);
            set_modal_state.set(ModalState::None);
        }
    });

    view! {
        <div class="space-y-6">
            <div class="grid grid-cols-2 gap-4">
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline">"Display Label"</label>
                    <input 
                        type="text" 
                        prop:value=label 
                        on:input=move |ev| set_label.set(event_target_value(&ev))
                        class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-2 jetbrains text-sm text-on-surface transition-all placeholder:text-outline-variant/30" 
                        placeholder="e.g. TERMS OF SERVICE"
                    />
                </div>
                <div class="flex flex-col gap-2">
                    <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline">"Weight (Order)"</label>
                    <input 
                        type="number" 
                        prop:value=display_order 
                        on:input=move |ev| set_display_order.set(event_target_value(&ev))
                        class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-2 jetbrains text-sm text-on-surface transition-all" 
                    />
                </div>
            </div>

            <div class="flex flex-col gap-2">
                <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline">"Target URL (Href)"</label>
                <input 
                    type="text" 
                    prop:value=href 
                    on:input=move |ev| set_href.set(event_target_value(&ev))
                    class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-2 jetbrains text-sm text-on-surface transition-all placeholder:text-outline-variant/30" 
                    placeholder="e.g. /terms"
                />
            </div>

            <div class="flex items-center gap-3 bg-surface-container/50 p-4 border border-outline-variant/20">
                <label class="relative inline-flex items-center cursor-pointer">
                    <input type="checkbox" 
                        class="sr-only peer" 
                        prop:checked=is_visible 
                        on:change=move |ev| set_is_visible.set(event_target_checked(&ev))
                    />
                    <div class="w-9 h-5 bg-outline-variant peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-surface after:border-outline-variant after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-primary"></div>
                    <span class="ml-3 text-xs jetbrains font-bold uppercase tracking-widest text-primary">
                        {move || if is_visible.get() { "VISIBLE ON SITE" } else { "HIDDEN ENTIRELY" }}
                    </span>
                </label>
            </div>

            <div class="flex justify-end gap-4 pt-4 border-t-2 border-outline-variant/30">
                <button 
                    on:click=move |_| set_modal_state.set(ModalState::None)
                    class="px-6 py-3 jetbrains text-xs font-bold tracking-widest uppercase text-outline hover:text-on-surface transition-colors"
                    disabled=loading
                >
                    "Cancel"
                </button>
                <button 
                    on:click=move |_| save_action.dispatch(())
                    class="bg-primary text-on-primary px-8 py-3 jetbrains text-xs font-bold tracking-widest uppercase hover:bg-primary-container transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
                    disabled=loading
                >
                    <Show when=move || loading.get()>
                        <span class="material-symbols-outlined animate-spin text-sm">"progress_activity"</span>
                    </Show>
                    {move || if is_edit { "COMMIT CHANGES" } else { "CREATE NODE" }}
                </button>
            </div>
        </div>
    }
}
