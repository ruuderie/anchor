use app::app::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    tracing::info!("tracing on frontend...");

    mount_to_body(move || {
        view! { <App/> }
    });
}