use leptos::*;
use std::time::Duration;

#[server(GetBlockHeight, "/api")]
pub async fn get_block_height() -> Result<u64, ServerFnError> {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let url = format!("https://mempool.space/api/v1/mining/blocks/timestamp/{}", now);
    let res = reqwest::get(&url).await?;
    let json: serde_json::Value = res.json().await?;
    let height = json["height"].as_u64().ok_or_else(|| ServerFnError::ServerError("Missing height".into()))?;
    Ok(height)
}

#[component]
pub fn Nav() -> impl IntoView {
    let (tick, set_tick) = create_signal(0);
    
    create_effect(move |_| {
        let handle = set_interval_with_handle(
            move || set_tick.update(|t| *t += 1),
            Duration::from_secs(60),
        ).ok();
        
        on_cleanup(move || {
            if let Some(h) = handle {
                h.clear();
            }
        });
    });

    let height_resource = create_resource(move || tick.get(), |_| get_block_height());
    let block_height = move || height_resource.get().unwrap_or(Ok(0)).unwrap_or(0);

    view! {
        <nav class="fixed top-0 left-0 w-full flex justify-between items-center px-[8.5rem] py-6 bg-white/80 dark:bg-slate-900/80 backdrop-blur-[20px] z-50">
            <a href="/" class="text-xl font-bold font-mono text-cyan-800 dark:text-cyan-400">
                "RUUDERIE_AI"
            </a>
            <div class="hidden md:flex items-center space-x-8">
                <a href="/resume" class="text-slate-600 dark:text-slate-400 font-medium hover:bg-slate-100/50 dark:hover:bg-slate-800/50 transition-colors">"EXPERIENCE"</a>
                <a href="/projects" class="text-slate-600 dark:text-slate-400 font-medium hover:bg-slate-100/50 dark:hover:bg-slate-800/50 transition-colors">"WORK"</a>
                <a href="/blog" class="text-slate-600 dark:text-slate-400 font-medium hover:bg-slate-100/50 dark:hover:bg-slate-800/50 transition-colors">"PROJECTS"</a>
                <a href="/resume" class="text-slate-600 dark:text-slate-400 font-medium hover:bg-slate-100/50 dark:hover:bg-slate-800/50 transition-colors">"RESUME"</a>
            </div>
            <div class="flex items-center space-x-6">
                <a href="/admin" class="material-symbols-outlined text-primary cursor-pointer hover:opacity-80 transition-opacity block">"terminal"</a>
                <a href=move || format!("https://mempool.space/block/{}", block_height()) target="_blank" rel="noopener noreferrer" class="bg-primary text-on-primary px-6 py-2 jetbrains text-xs font-bold tracking-wider hover:opacity-80 transition-opacity block whitespace-nowrap">
                    "BLOCK #" {block_height}
                </a>
            </div>
        </nav>
    }
}
