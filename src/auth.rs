use leptos::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub mod ssr {
    use url::Url;
    pub use webauthn_rs::prelude::*;
    
    pub fn get_webauthn() -> Webauthn {
        let rp_origin = Url::parse("http://localhost:3000").unwrap();
        let builder = WebauthnBuilder::new("localhost", &rp_origin).unwrap()
            .rp_name("RuudErie_ai");
        builder.build().unwrap()
    }
}

#[server(RegisterStart, "/api")]
pub async fn register_start(username: String) -> Result<String, ServerFnError> {
    use self::ssr::*;
    use leptos_axum::extract;
    use axum::Extension;
    use crate::state::AppState;

    let app_state = match extract::<Extension<AppState>>().await {
        Ok(state) => state,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    
    let user_exists: bool = match sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
        .bind(&username)
        .fetch_one(&app_state.pool)
        .await
    {
        Ok(v) => v,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&app_state.pool)
        .await
        .unwrap_or(0);
    
    if count > 0 && !user_exists {
        return Err(ServerFnError::ServerError("Registration locked. Admin already exists.".into()));
    }

    let user_unique_id = Uuid::new_v4();
    let webauthn = get_webauthn();
    let res = match webauthn.start_passkey_registration(user_unique_id.clone(), &username, &username, None) {
        Ok(r) => r,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    
    let (challenge, reg_state) = res;
    let reg_json = serde_json::to_value(&reg_state).unwrap();
    
    if let Err(e) = sqlx::query("INSERT INTO auth_challenges (id, challenge_data) VALUES ($1, $2)")
        .bind(user_unique_id)
        .bind(reg_json)
        .execute(&app_state.pool)
        .await
    {
        return Err(ServerFnError::ServerError(e.to_string()));
    }

    let challenge_json = serde_json::to_string(&challenge).unwrap();
    let payload = serde_json::json!({
        "challenge_id": user_unique_id,
        "options": challenge_json
    });

    Ok(payload.to_string())
}

#[server(RegisterFinish, "/api")]
pub async fn register_finish(username: String, challenge_id: Uuid, credential_json: String) -> Result<String, ServerFnError> {
    use self::ssr::*;
    use leptos_axum::extract;
    use axum::Extension;
    use crate::state::AppState;

    let app_state = match extract::<Extension<AppState>>().await {
        Ok(state) => state,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    
    let challenge_row: (serde_json::Value,) = match sqlx::query_as("SELECT challenge_data FROM auth_challenges WHERE id = $1")
        .bind(challenge_id)
        .fetch_one(&app_state.pool)
        .await
    {
        Ok(row) => row,
        Err(_) => return Err(ServerFnError::ServerError("Challenge expired or invalid".into())),
    };

    let reg_state: PasskeyRegistration = match serde_json::from_value(challenge_row.0) {
        Ok(s) => s,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    let credential: RegisterPublicKeyCredential = match serde_json::from_str(&credential_json) {
        Ok(c) => c,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    
    let webauthn = get_webauthn();
    let passkey = match webauthn.finish_passkey_registration(&credential, &reg_state) {
        Ok(p) => p,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    let session_token = Uuid::new_v4().to_string();
    let passkey_json = serde_json::to_value(&passkey).unwrap();

    if let Err(e) = sqlx::query("INSERT INTO users (username, passkey, session_token) VALUES ($1, $2, $3)")
        .bind(&username)
        .bind(passkey_json)
        .bind(&session_token)
        .execute(&app_state.pool)
        .await
    {
        return Err(ServerFnError::ServerError(e.to_string()));
    }

    sqlx::query("DELETE FROM auth_challenges WHERE id = $1").bind(challenge_id).execute(&app_state.pool).await.ok();

    use leptos_axum::ResponseOptions;
    let response = leptos::expect_context::<ResponseOptions>();
    let header_val = format!("session={}; HttpOnly; Path=/; SameSite=Strict", session_token);
    response.append_header(axum::http::header::SET_COOKIE, axum::http::HeaderValue::from_str(&header_val).unwrap());

    Ok("SUCCESS".to_string())
}

#[server(LoginStart, "/api")]
pub async fn login_start(username: String) -> Result<String, ServerFnError> {
    use self::ssr::*;
    use leptos_axum::extract;
    use axum::Extension;
    use crate::state::AppState;

    let app_state = match extract::<Extension<AppState>>().await {
        Ok(state) => state,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    
    let user_row: (serde_json::Value,) = match sqlx::query_as("SELECT passkey FROM users WHERE username = $1")
        .bind(&username)
        .fetch_one(&app_state.pool)
        .await
    {
        Ok(row) => row,
        Err(_) => return Err(ServerFnError::ServerError("User not found".into())),
    };

    let passkey: Passkey = match serde_json::from_value(user_row.0) {
        Ok(k) => k,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    let webauthn = get_webauthn();
    let res = match webauthn.start_passkey_authentication(&[passkey]) {
        Ok(r) => r,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    
    let (challenge, auth_state) = res;
    let auth_id = Uuid::new_v4();

    let auth_json = serde_json::to_value(&auth_state).unwrap();
    if let Err(e) = sqlx::query("INSERT INTO auth_challenges (id, challenge_data) VALUES ($1, $2)")
        .bind(auth_id)
        .bind(auth_json)
        .execute(&app_state.pool)
        .await
    {
        return Err(ServerFnError::ServerError(e.to_string()));
    }

    let challenge_json = serde_json::to_string(&challenge).unwrap();
    let payload = serde_json::json!({
        "challenge_id": auth_id,
        "options": challenge_json
    });

    Ok(payload.to_string())
}

#[server(LoginFinish, "/api")]
pub async fn login_finish(username: String, challenge_id: Uuid, auth_json: String) -> Result<String, ServerFnError> {
    use self::ssr::*;
    use leptos_axum::extract;
    use axum::Extension;
    use crate::state::AppState;

    let app_state = match extract::<Extension<AppState>>().await {
        Ok(state) => state,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    
    let user_row: (serde_json::Value,) = match sqlx::query_as("SELECT passkey FROM users WHERE username = $1")
        .bind(&username)
        .fetch_one(&app_state.pool)
        .await
    {
        Ok(row) => row,
        Err(_) => return Err(ServerFnError::ServerError("User not found".into())),
    };

    let challenge_row: (serde_json::Value,) = match sqlx::query_as("SELECT challenge_data FROM auth_challenges WHERE id = $1")
        .bind(challenge_id)
        .fetch_one(&app_state.pool)
        .await
    {
        Ok(row) => row,
        Err(_) => return Err(ServerFnError::ServerError("Challenge expired or invalid".into())),
    };

    let auth_state: PasskeyAuthentication = match serde_json::from_value(challenge_row.0) {
        Ok(s) => s,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    let credential: PublicKeyCredential = match serde_json::from_str(&auth_json) {
        Ok(c) => c,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    
    let webauthn = get_webauthn();
    let _auth_res = match webauthn.finish_passkey_authentication(&credential, &auth_state) {
        Ok(r) => r,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    let session_token = Uuid::new_v4().to_string();

    if let Err(e) = sqlx::query("UPDATE users SET session_token = $1 WHERE username = $2")
        .bind(&session_token)
        .bind(&username)
        .execute(&app_state.pool)
        .await
    {
        return Err(ServerFnError::ServerError(e.to_string()));
    }

    sqlx::query("DELETE FROM auth_challenges WHERE id = $1").bind(challenge_id).execute(&app_state.pool).await.ok();

    use leptos_axum::ResponseOptions;
    let response = leptos::expect_context::<ResponseOptions>();
    let header_val = format!("session={}; HttpOnly; Path=/; SameSite=Strict", session_token);
    response.append_header(axum::http::header::SET_COOKIE, axum::http::HeaderValue::from_str(&header_val).unwrap());

    Ok("SUCCESS".to_string())
}

#[server(CheckSession, "/api")]
pub async fn check_session() -> Result<bool, ServerFnError> {
    use leptos_axum::extract;
    use axum::http::HeaderMap;
    use axum_extra::extract::cookie::CookieJar;
    use axum::Extension;
    use crate::state::AppState;
    
    let headers = extract::<HeaderMap>().await.unwrap_or_default();
    let cookies = CookieJar::from_headers(&headers);
    let session_cookie = cookies.get("session");
    
    if let Some(cookie) = session_cookie {
        let app_state = match extract::<Extension<AppState>>().await {
            Ok(state) => state,
            Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
        };
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE session_token = $1)")
            .bind(cookie.value())
            .fetch_one(&app_state.pool)
            .await
            .unwrap_or(false);
        Ok(exists)
    } else {
        Ok(false)
    }
}
