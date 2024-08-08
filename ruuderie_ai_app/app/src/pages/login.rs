use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

#[component]
pub fn Login() -> impl IntoView {
    let (email, email_modifier) = create_signal(String::new());
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        // Handle form submission logic here (e.g., send to server)
        println!("Email: {}", email.get());
    };

    view! {
        <div class="container">
            <section class="section">
                <div class="box">
                    <h2 class="title is-3">Login</h2>
                    <form on:submit=on_submit>
                        <div class="field">
                            <label class="label">Email</label>
                            <div class="control">
                                <input
                                    class="input"
                                    type="email"
                                    placeholder="Your email"
                                    prop:value=email
                                    on:input=move |ev| {
                                        email_modifier.set(event_target_value(&ev));
                                    }
                                />
                            </div>
                        </div>
                        <div class="field">
                            <div class="control">
                                <button class="button is-primary" type="submit">Login</button>
                            </div>
                        </div>
                    </form>
                </div>
            </section>
        </div>
    }
}
