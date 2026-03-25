use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full border-t border-outline-variant/30 py-8 px-6 md:px-[8.5rem] bg-surface-container-low mt-auto flex flex-col md:flex-row justify-between items-center text-xs jetbrains text-outline">
            <div class="flex items-center space-x-4 mb-4 md:mb-0">
                <span>"© 2026 RUUD SALYM ERIE. ALL RIGHTS RESERVED."</span>
                <span class="text-on-surface-variant">"|"</span>
                <span class="text-surface-variant font-bold text-outline">"OPLYST INTERNATIONAL, LLC."</span>
            </div>
            <div class="flex items-center space-x-3">
                <span class="text-[0.65rem] tracking-widest uppercase text-on-surface-variant">"Engineered natively in"</span>
                <a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer" class="flex items-center opacity-70 hover:opacity-100 transition-opacity p-2 bg-surface-container hover:bg-surface-container-high rounded-sm">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" alt="Rust Logo" class="h-5 w-5 dark:invert" />
                    <span class="ml-2 font-bold text-on-surface tracking-widest">"RUST"</span>
                </a>
            </div>
        </footer>
    }
}
