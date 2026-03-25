use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <main class="pt-32 pb-24 px-6 md:px-[8.5rem]">
            // Hero Section
            <section class="grid grid-cols-1 md:grid-cols-12 gap-12 min-h-[716px] items-start">
                <div class="md:col-span-12 lg:col-span-8 flex flex-col items-start">
                    <div class="inline-block bg-surface-container-high px-3 py-1 jetbrains text-[0.625rem] font-medium tracking-widest text-on-surface-variant mb-8">
                        "VER: v1.0.4 // KERNEL_ACTIVE"
                    </div>
                    <h1 class="text-6xl md:text-[6rem] leading-[0.9] font-extrabold tracking-[-0.04em] text-primary mb-12 uppercase">
                        "Ruud Salym"<br/>"Erie."
                    </h1>
                    <p class="text-xl md:text-2xl font-medium tracking-tight text-on-surface-variant max-w-2xl leading-relaxed">
                        "SALESFORCE TECHNICAL ARCHITECT // SPECIALIZING IN "<span class="text-secondary">"ENTERPRISE CLOUD SOLUTIONS"</span>", LWC, APEX, AND RUST EXTERNAL MICROSERVICES."
                    </p>
                    <div class="mt-20 flex space-x-12 items-end">
                        <div class="flex flex-col">
                            <span class="jetbrains text-[0.65rem] text-outline mb-2 uppercase">"Current focus"</span>
                            <span class="text-sm font-bold text-on-surface">"AI Agent Swarms (Agentforce / CrewAI)"</span>
                        </div>
                        <div class="flex flex-col">
                            <span class="jetbrains text-[0.65rem] text-outline mb-2 uppercase">"Status"</span>
                            <div class="flex items-center space-x-2">
                                <div class="w-1.5 h-1.5 bg-secondary"></div>
                                <span class="text-sm font-bold text-on-surface uppercase tracking-wider">"Available for Critical Ops"</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="col-span-12 lg:col-span-4 space-y-8 mt-12 lg:mt-0">
                    <div class="bg-surface-container-low p-8 border-l-4 border-secondary flex flex-col justify-between aspect-square lg:aspect-auto lg:min-h-[400px]">
                        <div>
                            <span class="material-symbols-outlined text-secondary text-4xl mb-6">"format_quote"</span>
                            <p class="text-lg italic font-medium text-on-surface leading-snug">
                                "\"Architecture is not about drawing boxes; it's about defining the physics of the data flow.\""
                            </p>
                        </div>
                        <div class="mt-8 space-y-6">
                            <div class="space-y-2">
                                <div class="flex justify-between items-end">
                                    <span class="jetbrains text-[0.65rem] uppercase text-outline">"Architectural Delivery"</span>
                                    <span class="jetbrains text-[0.65rem] text-secondary">"100%"</span>
                                </div>
                                <div class="h-1 bg-surface-container-highest w-full overflow-hidden">
                                    <div class="h-full bg-secondary w-full"></div>
                                </div>
                            </div>
                            <div class="space-y-2">
                                <div class="flex justify-between items-end">
                                    <span class="jetbrains text-[0.65rem] uppercase text-outline">"Resource Overhead"</span>
                                    <span class="jetbrains text-[0.65rem] text-primary">"2.4%"</span>
                                </div>
                                <div class="h-1 bg-surface-container-highest w-full overflow-hidden">
                                    <div class="h-full bg-primary w-[2.4%]"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Lead Capture Section
            <section class="mt-32 flex flex-col items-center justify-center py-32 bg-surface-container-low relative">
                <div class="absolute top-0 left-0 w-full h-px bg-outline-variant/30"></div>
                <div class="max-w-xl w-full text-center space-y-12 px-6 shadow-none">
                    <div class="space-y-4">
                        <h2 class="text-4xl font-extrabold tracking-tight text-primary">"Request Tailored CV"</h2>
                        <p class="text-on-surface-variant font-medium">"Input your protocol for a mission-specific credentials package."</p>
                    </div>
                    <form class="space-y-8 w-full bg-transparent border-0 outline-none">
                        <div class="relative w-full group">
                            <label class="jetbrains text-[0.65rem] uppercase tracking-[0.1em] text-outline text-left block mb-2">"Registry Email Address"</label>
                            <input type="email" placeholder="user@organization.domain" class="w-full bg-transparent border-none border-b-2 border-outline-variant focus:border-primary focus:ring-0 px-0 py-4 jetbrains text-lg text-on-surface placeholder:text-outline-variant/50 transition-all rounded-none" />
                        </div>
                        <div class="space-y-4">
                            <button type="submit" class="w-full bg-secondary text-on-primary py-6 jetbrains font-bold text-sm tracking-[0.2em] uppercase hover:bg-on-secondary-fixed-variant transition-colors rounded-none outline-none border-none shadow-none">
                                "Initialize Retrieval"
                            </button>
                            <p class="jetbrains text-[0.625rem] text-outline uppercase tracking-widest">
                                "* Check your email to confirm the request parameters."
                            </p>
                        </div>
                    </form>
                </div>
            </section>
        </main>
    }
}
