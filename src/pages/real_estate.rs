use leptos::*;

#[server(HandleRealEstateLead, "/api")]
pub async fn handle_real_estate_lead(email: String, options: Vec<String>) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    
    let prefs_json = serde_json::to_value(&options).unwrap_or(serde_json::json!([]));
    
    let _ = sqlx::query("INSERT INTO mailing_list (email, list_type, preferences) VALUES ($1, $2, $3) ON CONFLICT (email) DO UPDATE SET preferences = $3")
        .bind(&email)
        .bind("real_estate")
        .bind(&prefs_json)
        .execute(&state.pool)
        .await;
        
    println!("NEW REAL ESTATE LEAD: {} requested {:?}", email, options);

    Ok(())
}

#[component]
pub fn RealEstate() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (selected_options, set_selected_options) = create_signal(std::collections::HashSet::<String>::new());
    let (submitted, set_submitted) = create_signal(false);

    let submit_action = create_action(move |_: &()| {
        let e = email.get_untracked();
        let opts: Vec<String> = selected_options.get_untracked().into_iter().collect();
        async move {
            let _ = handle_real_estate_lead(e, opts).await;
            set_submitted.set(true);
        }
    });

    let settings_resource = create_resource(|| (), |_| crate::pages::landing::get_site_settings());

    let options_res = create_memo(move |_| {
        let json_str = settings_resource.get()
            .unwrap_or(Ok(crate::pages::landing::SiteSettings::default()))
            .unwrap_or(crate::pages::landing::SiteSettings::default())
            .re_options_json;
        serde_json::from_str::<std::collections::HashMap<String, String>>(&json_str)
            .unwrap_or_else(|_| [
                ("buying".to_string(), "Buying a Home".to_string()),
                ("selling".to_string(), "Selling a Home".to_string()),
                ("loan".to_string(), "Getting a real estate investment loan".to_string()),
                ("networking".to_string(), "Connecting with other investors".to_string())
            ].into())
    });

    view! {
        <main class="pt-32 pb-24 px-6 md:px-[8.5rem]">
            <section class="max-w-4xl mx-auto items-start">
                <div class="inline-block bg-surface-container-high px-3 py-1 jetbrains text-[0.625rem] font-medium tracking-widest text-on-surface-variant mb-8 uppercase">
                    <Suspense fallback=move || view! { <span>"..."</span> }>
                        {move || settings_resource.get().unwrap_or(Ok(crate::pages::landing::SiteSettings::default())).unwrap_or(crate::pages::landing::SiteSettings::default()).real_estate_desc}
                    </Suspense>
                </div>
                <h1 class="text-5xl md:text-[5rem] leading-[0.9] font-extrabold tracking-[-0.04em] text-primary mb-8 uppercase">
                    <Suspense fallback=move || view! { <span>"Real Estate"<br/>"Ventures."</span> }>
                        {move || {
                            let title = settings_resource.get().unwrap_or(Ok(crate::pages::landing::SiteSettings::default())).unwrap_or(crate::pages::landing::SiteSettings::default()).real_estate_title;
                            view! { <span inner_html=title></span> }
                        }}
                    </Suspense>
                </h1>
                <p class="text-xl md:text-2xl font-medium tracking-tight text-on-surface-variant leading-relaxed mb-8">
                    <Suspense fallback=move || view! { <span>"..."</span> }>
                        {move || settings_resource.get().unwrap_or(Ok(crate::pages::landing::SiteSettings::default())).unwrap_or(crate::pages::landing::SiteSettings::default()).real_estate_desc}
                    </Suspense>
                </p>
                
                <div class="bg-surface-container-low p-8 border-l-4 border-primary my-12">
                    {move || if submitted.get() {
                        view! {
                            <div class="text-center space-y-6 py-8">
                                <span class="material-symbols-outlined text-secondary text-5xl">"check_circle"</span>
                                <h2 class="text-3xl font-extrabold tracking-tight text-primary">"CONNECTION ESTABLISHED"</h2>
                                <p class="text-on-surface-variant font-medium">"Your selections have been securely transmitted."</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="space-y-8">
                                <div class="space-y-2">
                                    <h3 class="text-2xl font-bold tracking-tight text-on-surface">
                                        <Suspense fallback=move || view! { <span>"..."</span> }>
                                            {move || settings_resource.get().unwrap_or(Ok(crate::pages::landing::SiteSettings::default())).unwrap_or(crate::pages::landing::SiteSettings::default()).re_lc_title}
                                        </Suspense>
                                    </h3>
                                    <p class="text-on-surface-variant">
                                        <Suspense fallback=move || view! { <span>"..."</span> }>
                                            {move || settings_resource.get().unwrap_or(Ok(crate::pages::landing::SiteSettings::default())).unwrap_or(crate::pages::landing::SiteSettings::default()).re_lc_desc}
                                        </Suspense>
                                    </p>
                                </div>
                                <div class="space-y-4 w-full bg-transparent border-0 outline-none">
                                    <div class="relative w-full group">
                                        <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline text-left block mb-2">
                                            <Suspense fallback=move || view! { <span>"..."</span> }>
                                                {move || settings_resource.get().unwrap_or(Ok(crate::pages::landing::SiteSettings::default())).unwrap_or(crate::pages::landing::SiteSettings::default()).re_lc_label}
                                            </Suspense>
                                        </label>
                                        <input type="email" prop:value=email on:input=move |ev| set_email.set(event_target_value(&ev)) 
                                            placeholder="..." 
                                            class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-4 jetbrains text-lg text-on-surface placeholder:text-outline-variant/50 transition-all rounded-none" />
                                    </div>
                                    <div class="space-y-4 text-left border border-outline-variant/30 p-6 bg-surface-container-lowest/50 mt-4">
                                        <Suspense fallback=move || view! { <span>"..."</span> }>
                                            {move || options_res.get().into_iter().map(|(key, label)| {
                                                view! {
                                                <label class="flex items-center space-x-3 cursor-pointer group">
                                                    <input type="checkbox" 
                                                        class="w-5 h-5 bg-transparent border-2 border-outline-variant text-primary focus:ring-primary focus:ring-offset-surface-container-low" 
                                                        on:change=move |ev| {
                                                            let k = key.clone();
                                                            if event_target_checked(&ev) {
                                                                set_selected_options.update(|set| { set.insert(k); });
                                                            } else {
                                                                set_selected_options.update(|set| { set.remove(&k); });
                                                            }
                                                        }
                                                    />
                                                    <span class="jetbrains text-sm text-on-surface group-hover:text-primary transition-colors">{label}</span>
                                                </label>
                                                }
                                            }).collect_view()}
                                        </Suspense>
                                    </div>
                                    <div class="pt-4">
                                        <button on:click=move |_| submit_action.dispatch(()) class="w-full bg-primary text-on-primary py-6 jetbrains font-bold text-sm tracking-[0.2em] uppercase hover:bg-primary-container transition-colors rounded-none outline-none border-none shadow-none">
                                            <Suspense fallback=move || view! { <span>"..."</span> }>
                                                {move || settings_resource.get().unwrap_or(Ok(crate::pages::landing::SiteSettings::default())).unwrap_or(crate::pages::landing::SiteSettings::default()).re_lc_btn}
                                            </Suspense>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    }}
                </div>
            </section>
        </main>
    }
}
