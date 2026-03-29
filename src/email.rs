use leptos::*;

#[server(SendEmail, "/api")]
pub async fn send_email(to_email: String, subject: String, body_html: String) -> Result<(), ServerFnError> {
    use std::env;
    let api_key = match env::var("RESEND_API_KEY") {
        Ok(k) if !k.is_empty() => k,
        _ => {
            println!("RESEND_API_KEY not set. Email not sent to: {}", to_email);
            return Ok(());
        }
    };
    
    let client = reqwest::Client::new();
    let from_email = env::var("RESEND_FROM_EMAIL").unwrap_or_else(|_| "Acme <onboarding@resend.dev>".to_string());
    let payload = serde_json::json!({
        "from": from_email,
        "to": [to_email],
        "subject": subject,
        "html": body_html
    });

    let res = client.post("https://api.resend.com/emails")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await;
        
    match res {
        Ok(r) if r.status().is_success() => Ok(()),
        Ok(r) => {
            let status = r.status();
            let text = r.text().await.unwrap_or_default();
            println!("Failed to send email: {} - {}", status, text);
            Err(ServerFnError::ServerError("Failed to send email.".into()))
        },
        Err(e) => {
            println!("Reqwest error sending email: {:?}", e);
            Err(ServerFnError::ServerError("Network error sending email.".into()))
        }
    }
}
