use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResumeProfile {
    pub id: i32,
    pub name: String,
    pub objective: Option<String>,
    pub is_public: bool,
    pub target_role: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_location: Option<String>,
    pub contact_link: Option<String>,
    pub category_visibility: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "resume_category_enum", rename_all = "lowercase"))]
pub enum ResumeCategory {
    Work,
    Education,
    Skill,
    Project,
    Language,
    Volunteer,
    Extracurricular,
    Hobby,
}

impl std::fmt::Display for ResumeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Work => "work",
            Self::Education => "education",
            Self::Skill => "skill",
            Self::Project => "project",
            Self::Language => "language",
            Self::Volunteer => "volunteer",
            Self::Extracurricular => "extracurricular",
            Self::Hobby => "hobby",
        };
        write!(f, "{}", text)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResumeEntry {
    pub id: i32,
    pub profile_id: i32,
    pub category: ResumeCategory,
    pub title: String,
    pub subtitle: Option<String>,
    pub date_range: Option<String>,
    pub bullets: Vec<String>,
    pub display_order: i32,
    pub is_visible: bool,
}

#[server(GetResumeProfiles, "/api")]
pub async fn get_resume_profiles() -> Result<Vec<ResumeProfile>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT id, name, objective, is_public, target_role, contact_email, contact_phone, contact_location, contact_link, category_visibility FROM resume_profiles ORDER BY id ASC")
        .fetch_all(&state.pool)
        .await?;
        
    let profiles = rows.into_iter().map(|row| {
        ResumeProfile {
            id: row.get("id"),
            name: row.get("name"),
            objective: row.get("objective"),
            is_public: row.try_get("is_public").unwrap_or(false),
            target_role: row.get("target_role"),
            contact_email: row.get("contact_email"),
            contact_phone: row.get("contact_phone"),
            contact_location: row.get("contact_location"),
            contact_link: row.get("contact_link"),
            category_visibility: row.get("category_visibility"),
        }
    }).collect();
    
    Ok(profiles)
}

#[server(GetResumeEntries, "/api")]
pub async fn get_resume_entries(profile_id: i32) -> Result<Vec<ResumeEntry>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT id, profile_id, category, title, subtitle, date_range, bullets, display_order, is_visible FROM resume_entries WHERE profile_id = $1 ORDER BY display_order ASC")
        .bind(profile_id)
        .fetch_all(&state.pool)
        .await?;
        
    let items = rows.into_iter().map(|row| {
        let bullets_val: serde_json::Value = row.get("bullets");
        let bullets: Vec<String> = serde_json::from_value(bullets_val).unwrap_or_default();
        ResumeEntry {
            id: row.get("id"),
            profile_id: row.get("profile_id"),
            category: row.get("category"),
            title: row.get("title"),
            subtitle: row.get("subtitle"),
            date_range: row.get("date_range"),
            bullets,
            display_order: row.get("display_order"),
            is_visible: row.get("is_visible"),
        }
    }).collect();
    
    Ok(items)
}

#[server(AddResumeProfile, "/api")]
pub async fn add_resume_profile(
    name: String, objective: Option<String>, is_public: bool, target_role: Option<String>,
    contact_email: Option<String>, contact_phone: Option<String>, contact_location: Option<String>,
    contact_link: Option<String>, category_visibility: serde_json::Value
) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;

    sqlx::query("INSERT INTO resume_profiles (name, objective, is_public, target_role, contact_email, contact_phone, contact_location, contact_link, category_visibility) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(name).bind(objective).bind(is_public).bind(target_role)
        .bind(contact_email).bind(contact_phone).bind(contact_location).bind(contact_link).bind(category_visibility)
        .execute(&state.pool).await?;
        
    Ok(())
}

#[server(UpdateResumeProfile, "/api")]
pub async fn update_resume_profile(
    id: i32, name: String, objective: Option<String>, is_public: bool, target_role: Option<String>,
    contact_email: Option<String>, contact_phone: Option<String>, contact_location: Option<String>,
    contact_link: Option<String>, category_visibility: serde_json::Value
) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;

    sqlx::query("UPDATE resume_profiles SET name = $1, objective = $2, is_public = $3, target_role = $4, contact_email = $5, contact_phone = $6, contact_location = $7, contact_link = $8, category_visibility = $9 WHERE id = $10")
        .bind(name).bind(objective).bind(is_public).bind(target_role)
        .bind(contact_email).bind(contact_phone).bind(contact_location).bind(contact_link).bind(category_visibility).bind(id)
        .execute(&state.pool).await?;
        
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

#[server(AddResumeEntry, "/api")]
pub async fn add_resume_entry(
    profile_id: i32, category: ResumeCategory, title: String, subtitle: Option<String>,
    date_range: Option<String>, bullets: Vec<String>, display_order: i32, is_visible: bool
) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let bullets_json = serde_json::to_value(&bullets).unwrap_or(serde_json::json!([]));
    
    sqlx::query("INSERT INTO resume_entries (profile_id, category, title, subtitle, date_range, bullets, display_order, is_visible) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(profile_id).bind(category).bind(title).bind(subtitle).bind(date_range)
        .bind(bullets_json).bind(display_order).bind(is_visible)
        .execute(&state.pool).await?;
        
    Ok(())
}

#[server(UpdateResumeEntry, "/api")]
pub async fn update_resume_entry(
    id: i32, category: ResumeCategory, title: String, subtitle: Option<String>,
    date_range: Option<String>, bullets: Vec<String>, display_order: i32, is_visible: bool
) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let bullets_json = serde_json::to_value(&bullets).unwrap_or(serde_json::json!([]));
    
    sqlx::query("UPDATE resume_entries SET category = $1, title = $2, subtitle = $3, date_range = $4, bullets = $5, display_order = $6, is_visible = $7 WHERE id = $8")
        .bind(category).bind(title).bind(subtitle).bind(date_range)
        .bind(bullets_json).bind(display_order).bind(is_visible).bind(id)
        .execute(&state.pool).await?;
        
    Ok(())
}

#[server(DeleteResumeEntry, "/api")]
pub async fn delete_resume_entry(id: i32) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    sqlx::query("DELETE FROM resume_entries WHERE id = $1").bind(id).execute(&state.pool).await?;
    Ok(())
}

pub fn latex_escape(s: &str) -> String {
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

pub fn generate_latex_string(
    profile: &ResumeProfile,
    entries: &[ResumeEntry]
) -> String {
    let target_role_str = profile.target_role.clone().unwrap_or_default();
    let objective_str = profile.objective.clone().unwrap_or_default();
    
    let mut contact_parts = Vec::new();
    if let Some(ref email) = profile.contact_email { if !email.is_empty() { contact_parts.push(latex_escape(email)); } }
    if let Some(ref phone) = profile.contact_phone { if !phone.is_empty() { contact_parts.push(latex_escape(phone)); } }
    if let Some(ref loc) = profile.contact_location { if !loc.is_empty() { contact_parts.push(latex_escape(loc)); } }
    if let Some(ref link) = profile.contact_link { if !link.is_empty() { contact_parts.push(latex_escape(link)); } }
    let contact_str = contact_parts.join(" \\textbar{} ");

    let mut tex_content = format!(
        r#"
\documentclass[11pt,letterpaper]{{article}}
\usepackage[utf8]{{inputenc}}
\usepackage[left=0.5in,top=0.5in,right=0.5in,bottom=0.5in]{{geometry}}
\usepackage{{enumitem}}
\usepackage{{titlesec}}
\usepackage{{charter}}

\titleformat{{\section}}
  {{\normalfont\Large\bfseries}}
  {{}}{{0em}}
  {{}}[\titlerule]
\titlespacing*{{\section}}{{0pt}}{{2ex}}{{1ex}}

\begin{{document}}
\pagestyle{{empty}}
\begin{{center}}
    {{\Huge \textbf{{{}}}}} \\
    \vspace{{0.1in}}
    {{\Large \textsc{{{}}}}} \\
    \vspace{{0.05in}}
    {{\normalsize \texttt{{{}}}}}
\end{{center}}
\vspace{{0.1in}}
"#,
        latex_escape(&profile.name),
        latex_escape(&target_role_str),
        contact_str
    );

    if !objective_str.is_empty() {
        tex_content.push_str(&format!(
            "\\section*{{Executive Summary}}\n\\noindent {}\n\n",
            latex_escape(&objective_str)
        ));
    }

    let is_cat_visible = |cat: &ResumeCategory| -> bool {
        profile.category_visibility.get(cat.to_string())
            .and_then(|v| v.as_bool()).unwrap_or(true)
    };

    let categories_to_render = vec![
        (ResumeCategory::Work, "Experience"),
        (ResumeCategory::Education, "Education"),
        (ResumeCategory::Skill, "Skills"),
        (ResumeCategory::Project, "Projects"),
        (ResumeCategory::Language, "Languages"),
        (ResumeCategory::Volunteer, "Volunteering"),
        (ResumeCategory::Extracurricular, "Extracurriculars"),
        (ResumeCategory::Hobby, "Hobbies"),
    ];

    for (cat_enum, section_title) in categories_to_render {
        if !is_cat_visible(&cat_enum) { continue; }
        
        let cat_entries: Vec<_> = entries.iter()
            .filter(|e| e.category == cat_enum && e.is_visible)
            .collect();
            
        if cat_entries.is_empty() { continue; }

        tex_content.push_str(&format!("\\section*{{{}}}\n", section_title));

        for entry in cat_entries {
            let title = latex_escape(&entry.title);
            let date = latex_escape(&entry.date_range.clone().unwrap_or_default());
            let subtitle = latex_escape(&entry.subtitle.clone().unwrap_or_default());

            tex_content.push_str(&format!(
                "\\noindent \\textbf{{{}}} \\hfill {} \\\\\n", title, date
            ));
            
            if !subtitle.is_empty() {
                tex_content.push_str(&format!("\\textit{{{}}} \\vspace{{0.05in}}\n", subtitle));
            } else {
                tex_content.push_str("\\vspace{0.05in}\n");
            }

            if !entry.bullets.is_empty() {
                tex_content.push_str("\\begin{itemize}[leftmargin=*,noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt]\n");
                for bullet in &entry.bullets {
                    tex_content.push_str(&format!("\\item {}\n", latex_escape(bullet)));
                }
                tex_content.push_str("\\end{itemize}\n");
            }
            tex_content.push_str("\\vspace{0.15in}\n\n");
        }
    }

    tex_content.push_str("\\end{document}\n");
    tex_content
}

#[server(DownloadResume, "/api")]
pub async fn download_resume(profile_id: i32) -> Result<Vec<u8>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;

    if profile_id <= 0 {
        return Err(ServerFnError::ServerError("Invalid Profile ID".into()));
    }

    let profile_row = sqlx::query("SELECT id, name, objective, is_public, target_role, contact_email, contact_phone, contact_location, contact_link, category_visibility FROM resume_profiles WHERE id = $1")
        .bind(profile_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| -> ServerFnError { ServerFnError::ServerError("Profile not found".into()) })?;

    let profile = ResumeProfile {
        id: profile_row.get("id"),
        name: profile_row.get("name"),
        objective: profile_row.get("objective"),
        is_public: profile_row.try_get("is_public").unwrap_or(false),
        target_role: profile_row.get("target_role"),
        contact_email: profile_row.get("contact_email"),
        contact_phone: profile_row.get("contact_phone"),
        contact_location: profile_row.get("contact_location"),
        contact_link: profile_row.get("contact_link"),
        category_visibility: profile_row.get("category_visibility"),
    };

    let entries_rows = sqlx::query("SELECT id, profile_id, category, title, subtitle, date_range, bullets, display_order, is_visible FROM resume_entries WHERE profile_id = $1 ORDER BY display_order ASC")
        .bind(profile_id).fetch_all(&state.pool).await?;

    let entries: Vec<ResumeEntry> = entries_rows.into_iter().map(|row| {
        let bullets_val: serde_json::Value = row.get("bullets");
        let bullets: Vec<String> = serde_json::from_value(bullets_val).unwrap_or_default();
        ResumeEntry {
            id: row.get("id"),
            profile_id: row.get("profile_id"),
            category: row.get("category"),
            title: row.get("title"),
            subtitle: row.get("subtitle"),
            date_range: row.get("date_range"),
            bullets,
            display_order: row.get("display_order"),
            is_visible: row.get("is_visible"),
        }
    }).collect();

    let tex_content = generate_latex_string(&profile, &entries);

    let tex_path = format!("/tmp/resume_output_{}.tex", uuid::Uuid::new_v4());
    let pdf_path = tex_path.replace(".tex", ".pdf");

    if let Err(e) = std::fs::write(&tex_path, &tex_content) {
        return Err(ServerFnError::ServerError(format!("Failed to write TEX source: {}", e).into()));
    }

    let mut command = std::process::Command::new("tectonic");
    command.current_dir("/tmp");
    command.arg("-X").arg("compile").arg(&tex_path);

    let status = command.output().map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("Failed to execute tectonic: {}", e).into()) })?;

    if !status.status.success() {
        let output2 = std::process::Command::new("pdflatex")
            .current_dir("/tmp")
            .arg("-interaction=nonstopmode")
            .arg(&tex_path)
            .output()
            .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("Failed to execute pdflatex compiler: {}", e).into()) })?;

        if !output2.status.success() {
            return Err(ServerFnError::ServerError(format!("Latex Compilation Error").into()));
        }
    }

    let pdf_bytes = std::fs::read(&pdf_path).map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("Failed to read compiled PDF: {}", e).into()) })?;
    
    let _ = std::fs::remove_file(&tex_path);
    let _ = std::fs::remove_file(&pdf_path);

    Ok(pdf_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_escape() {
        let input = "M&A $100% #1 _test_ {foo} ~bar^";
        let expected = "M\\&A \\$100\\% \\#1 \\_test\\_ \\{foo\\} \\textasciitilde{}bar\\textasciicircum{}";
        assert_eq!(latex_escape(input), expected);
    }

    #[test]
    fn test_generate_latex_string_formats_correctly() {
        let profile = ResumeProfile {
            id: 1,
            name: "John Doe".into(),
            objective: Some("Software Dev".into()),
            is_public: true,
            target_role: Some("Engineer".into()),
            contact_email: Some("john@test.com".into()),
            contact_phone: None,
            contact_location: None,
            contact_link: None,
            category_visibility: serde_json::json!({"work": true, "education": true}),
        };
        let entries = vec![ResumeEntry {
            id: 1,
            profile_id: 1,
            category: ResumeCategory::Work,
            title: "Developer".into(),
            subtitle: Some("Tech Corp".into()),
            date_range: Some("2020-2022".into()),
            bullets: vec!["Did things & stuff".into()],
            display_order: 0,
            is_visible: true,
        }];
        
        let tex = generate_latex_string(&profile, &entries);
        
        assert!(tex.contains("Software Dev"));
        assert!(tex.contains("Tech Corp"));
        assert!(tex.contains("Did things \\& stuff"));
        assert!(tex.contains("\\section*{Experience}"));
    }

    #[test]
    fn test_generate_latex_string_filters_and_masks() {
        let profile = ResumeProfile {
            id: 1,
            name: "Jane Doe".into(),
            objective: Some("Hidden Test".into()),
            is_public: true,
            target_role: None,
            contact_email: None,
            contact_phone: None,
            contact_location: None,
            contact_link: None,
            category_visibility: serde_json::json!({"work": true, "education": false}),
        };
        let entries = vec![
            ResumeEntry {
                id: 1,
                profile_id: 1,
                category: ResumeCategory::Work,
                title: "Hidden Job".into(),
                subtitle: None,
                date_range: None,
                bullets: vec![],
                display_order: 0,
                is_visible: false, // Target should hide inherently
            },
            ResumeEntry {
                id: 2,
                profile_id: 1,
                category: ResumeCategory::Education,
                title: "Hidden Education".into(),
                subtitle: None,
                date_range: None,
                bullets: vec![],
                display_order: 1,
                is_visible: true, // Should hide because master JSON hides Education
            }
        ];
        
        let tex = generate_latex_string(&profile, &entries);
        
        assert!(!tex.contains("Hidden Job"));
        assert!(!tex.contains("Hidden Education"));
    }
}
