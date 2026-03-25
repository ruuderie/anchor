use leptos::*;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <main class="pt-32 pb-24 px-6 md:px-[8.5rem] bg-surface-container-low min-h-screen">
            <header class="mb-24 max-w-4xl">
                <div class="inline-block bg-secondary-container/20 px-3 py-1 mb-6">
                    <span class="font-label text-[0.6875rem] text-secondary font-bold tracking-tighter">"INDEX_REF_02 // LOGS"</span>
                </div>
                <h1 class="text-5xl md:text-7xl font-extrabold text-primary tracking-[-0.02em] leading-none mb-8">
                    "SYSTEM LOGS"
                </h1>
                <p class="text-lg text-on-surface-variant max-w-2xl leading-relaxed">
                    "Internal documentation on architecture decisions, performance profiling, and distributed systems theory."
                </p>
            </header>

            <div class="space-y-12 max-w-4xl">
                <article class="bg-surface-container p-8 hover:bg-surface-container-high transition-colors group cursor-pointer border-l-4 border-transparent hover:border-secondary">
                    <div class="flex justify-between items-start mb-4">
                        <h3 class="text-2xl font-bold text-primary group-hover:text-secondary transition-colors">"Anatomy of a Microsecond: Navigating the Kernel"</h3>
                        <span class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider whitespace-nowrap pt-1 ml-4">"2026.03.14 // V_2.4"</span>
                    </div>
                    <p class="text-on-surface-variant leading-relaxed text-sm mb-6 max-w-2xl">
                        "An exploration of context-switching overhead in high-frequency trading platforms and how custom kernel modules can achieve 4μs round-trip latency."
                    </p>
                    <div class="flex gap-4">
                        <span class="bg-surface-container-highest px-3 py-1 jetbrains text-[0.65rem] font-bold text-on-surface-variant uppercase">"Kernel_Ops"</span>
                        <span class="bg-surface-container-highest px-3 py-1 jetbrains text-[0.65rem] font-bold text-on-surface-variant uppercase">"Low_Latency"</span>
                    </div>
                </article>

                <article class="bg-surface-container p-8 hover:bg-surface-container-high transition-colors group cursor-pointer border-l-4 border-transparent hover:border-secondary">
                    <div class="flex justify-between items-start mb-4">
                        <h3 class="text-2xl font-bold text-primary group-hover:text-secondary transition-colors">"Distributed State: Why CAP Theorem is just the Beginning"</h3>
                        <span class="jetbrains text-[0.65rem] uppercase text-outline tracking-wider whitespace-nowrap pt-1 ml-4">"2026.02.28 // V_1.1"</span>
                    </div>
                    <p class="text-on-surface-variant leading-relaxed text-sm mb-6 max-w-2xl">
                        "Evaluating consensus algorithms (Raft, Paxos) in the context of geographic-scale distributed datasets with unreliable wide-area networks."
                    </p>
                    <div class="flex gap-4">
                        <span class="bg-surface-container-highest px-3 py-1 jetbrains text-[0.65rem] font-bold text-on-surface-variant uppercase">"Architecture"</span>
                        <span class="bg-surface-container-highest px-3 py-1 jetbrains text-[0.65rem] font-bold text-on-surface-variant uppercase">"Consensus"</span>
                    </div>
                </article>
            </div>
        </main>
    }
}
