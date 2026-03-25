use leptos::*;

#[server(SendEmail, "/api")]
pub async fn send_email(_email: String) -> Result<(), ServerFnError> {
    // Stub definition for compiling without key requirements
    Ok(())
}
