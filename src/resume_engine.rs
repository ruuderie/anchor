use leptos::*;
use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResumeProfile {
    pub id: i32,
    pub name: String,
    pub biography: String,
    pub excluded_tags: Vec<String>,
    pub anonymous_companies: std::collections::HashMap<String, String>,
}

#[server(GetResumeProfiles, "/api")]
pub async fn get_resume_profiles() -> Result<Vec<ResumeProfile>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT id, name, biography, excluded_tags, anonymous_companies FROM resume_profiles ORDER BY id ASC")
        .fetch_all(&state.pool)
        .await?;
        
    let profiles = rows.into_iter().map(|row| {
        let excluded_json: serde_json::Value = row.get("excluded_tags");
        let anon_json: serde_json::Value = row.get("anonymous_companies");
        
        ResumeProfile {
            id: row.get("id"),
            name: row.get("name"),
            biography: row.get("biography"),
            excluded_tags: serde_json::from_value(excluded_json).unwrap_or_default(),
            anonymous_companies: serde_json::from_value(anon_json).unwrap_or_default(),
        }
    }).collect();
    
    Ok(profiles)
}

#[server(AddResumeProfile, "/api")]
pub async fn add_resume_profile(name: String, biography: String, excluded_tags: Vec<String>, anonymous_companies: std::collections::HashMap<String, String>) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let excluded = serde_json::to_value(excluded_tags).unwrap_or_default();
    let anon = serde_json::to_value(anonymous_companies).unwrap_or_default();
    sqlx::query("INSERT INTO resume_profiles (name, biography, excluded_tags, anonymous_companies) VALUES ($1, $2, $3, $4)")
        .bind(name).bind(biography).bind(excluded).bind(anon).execute(&state.pool).await?;
    Ok(())
}

#[server(UpdateResumeProfile, "/api")]
pub async fn update_resume_profile(id: i32, name: String, biography: String, excluded_tags: Vec<String>, anonymous_companies: std::collections::HashMap<String, String>) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let excluded = serde_json::to_value(excluded_tags).unwrap_or_default();
    let anon = serde_json::to_value(anonymous_companies).unwrap_or_default();
    sqlx::query("UPDATE resume_profiles SET name = $1, biography = $2, excluded_tags = $3, anonymous_companies = $4 WHERE id = $5")
        .bind(name).bind(biography).bind(excluded).bind(anon).bind(id).execute(&state.pool).await?;
    Ok(())
}

#[server(DeleteResumeProfile, "/api")]
pub async fn delete_resume_profile(id: i32) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("DELETE FROM resume_profiles WHERE id = $1").bind(id).execute(&state.pool).await?;
    Ok(())
}

fn latex_escape(s: &str) -> String {
    s.replace("&", "\\&")
     .replace("%", "\\%")
     .replace("$", "\\$")
     .replace("#", "\\#")
     .replace("_", "\\_")
     .replace("{", "\\{")
     .replace("}", "\\}")
     .replace("~", "\\textasciitilde{}")
     .replace("^", "\\textasciicircum{}")
}

#[server(DownloadResume, "/api")]
pub async fn download_resume(profile_id: i32) -> Result<Vec<u8>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;

    let settings = crate::pages::landing::get_site_settings().await.unwrap_or_default();

    // If profile_id == 0, use defaults
    let (bio, excluded_tags, anonymous_companies) = if profile_id > 0 {
        let profile_row = sqlx::query("SELECT biography, excluded_tags, anonymous_companies FROM resume_profiles WHERE id = $1")
            .bind(profile_id)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| ServerFnError::ServerError("Profile not found".into()))?;

        let bio: String = profile_row.get("biography");
        let excluded_json: serde_json::Value = profile_row.get("excluded_tags");
        let anon_json: serde_json::Value = profile_row.get("anonymous_companies");
        let excluded_tags: Vec<String> = serde_json::from_value(excluded_json).unwrap_or_default();
        let anonymous_companies: std::collections::HashMap<String, String> = serde_json::from_value(anon_json).unwrap_or_default();
        (bio, excluded_tags, anonymous_companies)
    } else {
        ("Specializing in Enterprise Cloud Solutions, APEX, and Rust External Microservices. Dedicated to translating complex systems into immutable data flows.".to_string(), vec![], std::collections::HashMap::new())
    };

    let jobs = crate::pages::resume::get_jobs().await.unwrap_or_default();
    let projects = crate::pages::projects::get_projects().await.unwrap_or_default();
    
    let mut tex_content = format!(
        r#"
\documentclass[11pt,a4paper]{{article}}
\usepackage[utf8]{{inputenc}}
\usepackage{{geometry}}
\usepackage{{enumitem}}
\geometry{{margin=0.8in}}
\begin{{document}}
\begin{{center}}
    \Huge \textbf{{{}}} \\
    \vspace{{0.5cm}}
    \Large \textsc{{{}}} \\
    \vspace{{0.5cm}}
    \normalsize \texttt{{{}}}
\end{{center}}
\vspace{{0.5cm}}

\section*{{Executive Summary}}
{}

"#,
        latex_escape(&settings.site_title),
        latex_escape(&settings.hero_subtitle),
        latex_escape(&settings.hero_quote),
        latex_escape(&bio)
    );

    tex_content.push_str("\\section*{Experience}\n");

    for job in jobs.iter().rev() {
        if job.tags.iter().any(|t| excluded_tags.contains(t)) {
            continue;
        }
        
        let mut company_name = latex_escape(&job.company);
        if let Some(anon_name) = anonymous_companies.get(&job.company) {
            company_name = latex_escape(anon_name);
        }

        let date_str = if job.hide_date { String::new() } else { latex_escape(&job.date_range) };

        tex_content.push_str(&format!(
            "\\noindent \\textbf{{{}}} \\hfill {} \\\\\n\\textit{{{}}} \\\\\n",
            latex_escape(&job.role),
            date_str,
            company_name
        ));

        tex_content.push_str("\\begin{itemize}[noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt]\n");
        for bullet in &job.bullets {
            tex_content.push_str(&format!("\\item {}\n", latex_escape(bullet)));
        }
        tex_content.push_str("\\end{itemize}\n\\vspace{0.3cm}\n\n");
    }

    tex_content.push_str("\\section*{Projects}\n");
    for proj in projects.iter().rev() {
        if proj.tags.iter().any(|t| excluded_tags.contains(t)) {
            continue;
        }

        tex_content.push_str(&format!(
            "\\noindent \\textbf{{{}}} \\hfill {} \\\\\n",
            latex_escape(&proj.title),
            latex_escape(&proj.date_range)
        ));

        tex_content.push_str("\\begin{itemize}[noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt]\n");
        for bullet in &proj.bullets {
            tex_content.push_str(&format!("\\item {}\n", latex_escape(bullet)));
        }
        tex_content.push_str("\\end{itemize}\n\\vspace{0.3cm}\n\n");
    }

    tex_content.push_str("\\end{document}\n");

    let tex_path = "/tmp/resume_output.tex";
    let pdf_path = "/tmp/resume_output.pdf";

    if let Err(e) = std::fs::write(tex_path, tex_content) {
        return Err(ServerFnError::ServerError(format!("Failed to write TEX source: {}", e).into()));
    }

    let mut command = std::process::Command::new("tectonic");
    command.current_dir("/tmp");
    command.arg("-X").arg("compile").arg("resume_output.tex");

    let status = command.output().map_err(|e| ServerFnError::ServerError(format!("Failed to execute tectonic: {}", e).into()))?;

    if !status.status.success() {
        let output2 = std::process::Command::new("pdflatex")
            .current_dir("/tmp")
            .arg("-interaction=nonstopmode")
            .arg("resume_output.tex")
            .output()
            .map_err(|e| ServerFnError::ServerError(format!("Failed to execute pdflatex compiler: {}", e).into()))?;

        if !output2.status.success() {
            return Err(ServerFnError::ServerError(format!("Latex Compilation Error").into()));
        }
    }

    let pdf_bytes = std::fs::read(pdf_path).map_err(|e| ServerFnError::ServerError(format!("Failed to read compiled PDF: {}", e).into()))?;
    Ok(pdf_bytes)
}
