use app::app::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    tracing::info!("tracing on frontend...");

    mount_to_body(move || {
        view! { <App/> }
    });
}