use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResumeProfile {
    pub id: i32,
    pub name: String,
    pub biography: String,
    pub is_public: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResumeProfileItem {
    pub profile_id: i32,
    pub item_type: String, // 'job', 'project', 'cert'
    pub item_id: i32,
    pub custom_name: Option<String>,
}

#[server(GetResumeProfiles, "/api")]
pub async fn get_resume_profiles() -> Result<Vec<ResumeProfile>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT id, name, biography, is_public FROM resume_profiles ORDER BY id ASC")
        .fetch_all(&state.pool)
        .await?;
        
    let profiles = rows.into_iter().map(|row| {
        ResumeProfile {
            id: row.get("id"),
            name: row.get("name"),
            biography: row.get("biography"),
            is_public: row.try_get("is_public").unwrap_or(false),
        }
    }).collect();
    
    Ok(profiles)
}

#[server(GetResumeProfileItems, "/api")]
pub async fn get_resume_profile_items(profile_id: i32) -> Result<Vec<ResumeProfileItem>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::Row;
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;
    let rows = sqlx::query("SELECT profile_id, item_type, item_id, custom_name FROM resume_profile_items WHERE profile_id = $1")
        .bind(profile_id)
        .fetch_all(&state.pool)
        .await?;
        
    let items = rows.into_iter().map(|row| {
        ResumeProfileItem {
            profile_id: row.get("profile_id"),
            item_type: row.get("item_type"),
            item_id: row.get("item_id"),
            custom_name: row.get("custom_name"),
        }
    }).collect();
    
    Ok(items)
}

#[server(AddResumeProfile, "/api")]
pub async fn add_resume_profile(name: String, biography: String, is_public: bool, items: Vec<ResumeProfileItem>) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;

    let mut tx = state.pool.begin().await?;

    let row = sqlx::query("INSERT INTO resume_profiles (name, biography, is_public) VALUES ($1, $2, $3) RETURNING id")
        .bind(&name).bind(&biography).bind(is_public).fetch_one(&mut *tx).await?;
    let new_id: i32 = sqlx::Row::get(&row, "id");

    for item in items {
        sqlx::query("INSERT INTO resume_profile_items (profile_id, item_type, item_id, custom_name) VALUES ($1, $2, $3, $4)")
            .bind(new_id).bind(&item.item_type).bind(item.item_id).bind(&item.custom_name)
            .execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(())
}

#[server(UpdateResumeProfile, "/api")]
pub async fn update_resume_profile(id: i32, name: String, biography: String, is_public: bool, items: Vec<ResumeProfileItem>) -> Result<(), ServerFnError> {
    use crate::auth::check_session;
    use axum::Extension;
    use leptos_axum::extract;
    if !check_session().await.unwrap_or(false) { return Err(ServerFnError::ServerError("Unauthorized".into())); }
    let Extension(state) = extract::<Extension<crate::state::AppState>>().await?;

    let mut tx = state.pool.begin().await?;

    sqlx::query("UPDATE resume_profiles SET name = $1, biography = $2, is_public = $3 WHERE id = $4")
        .bind(&name).bind(&biography).bind(is_public).bind(id).execute(&mut *tx).await?;

    sqlx::query("DELETE FROM resume_profile_items WHERE profile_id = $1")
        .bind(id).execute(&mut *tx).await?;

    for item in items {
        sqlx::query("INSERT INTO resume_profile_items (profile_id, item_type, item_id, custom_name) VALUES ($1, $2, $3, $4)")
            .bind(id).bind(&item.item_type).bind(item.item_id).bind(&item.custom_name)
            .execute(&mut *tx).await?;
    }

    tx.commit().await?;
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
    settings_site_title: &str,
    settings_hero_subtitle: &str,
    settings_hero_quote: &str,
    bio: &str,
    jobs: &[crate::pages::resume::JobRecord],
    projects: &[crate::pages::projects::ProjectRecord],
    profile_items: &[ResumeProfileItem]
) -> String {
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

\section*{{Executive Summary}}
\noindent {}

"#,
        latex_escape(settings_site_title),
        latex_escape(settings_hero_subtitle),
        latex_escape(settings_hero_quote),
        latex_escape(bio)
    );

    // Render Jobs
    let profile_has_jobs = profile_items.iter().any(|i| i.item_type == "job") || profile_items.is_empty();
    if profile_has_jobs && !jobs.is_empty() {
        let mut standard_jobs = Vec::new();
        // Use a vector of tuples to maintain deterministic insertion order instead of a random Hashmap
        let mut c2c_groups: Vec<(String, Vec<&crate::pages::resume::JobRecord>)> = Vec::new();

        for job in jobs.iter().rev() {
            let item_cfg = if profile_items.is_empty() { None } else { profile_items.iter().find(|i| i.item_type == "job" && i.item_id == job.id) };
            if profile_items.is_empty() || item_cfg.is_some() {
                if job.employment_type == crate::pages::resume::JobType::CorpToCorp {
                    let parent = job.parent_company.clone().unwrap_or_else(|| "Independent Consulting".to_string());
                    if let Some(group) = c2c_groups.iter_mut().find(|(p, _)| p == &parent) {
                        group.1.push(job);
                    } else {
                        c2c_groups.push((parent, vec![job]));
                    }
                } else {
                    standard_jobs.push((job, item_cfg));
                }
            }
        }

        if !standard_jobs.is_empty() {
            tex_content.push_str("\\section*{Experience}\n");
            for (job, item_cfg) in standard_jobs {
                let mut company_name = job.company.clone();
                if let Some(cfg) = item_cfg {
                    if let Some(ref custom) = cfg.custom_name {
                        if !custom.is_empty() {
                            company_name = custom.clone();
                        }
                    }
                }
                
                let date_str = if job.hide_date { String::new() } else { job.date_range.clone() };

                tex_content.push_str(&format!(
                    "\\noindent \\textbf{{{}}} \\hfill {} \\\\\n\\textit{{{}}} \\vspace{{0.05in}}\n",
                    latex_escape(&company_name),
                    latex_escape(&date_str),
                    latex_escape(&job.role)
                ));

                tex_content.push_str("\\begin{itemize}[leftmargin=*,noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt]\n");
                for bullet in &job.bullets {
                    tex_content.push_str(&format!("\\item {}\n", latex_escape(bullet)));
                }
                tex_content.push_str("\\end{itemize}\n\\vspace{0.15in}\n\n");
            }
        }

        if !c2c_groups.is_empty() {
            tex_content.push_str("\\section*{Consulting Experience}\n");
            for (parent_company, c2c_list) in c2c_groups {
                tex_content.push_str(&format!("\\noindent {{\\Large \\textbf{{{}}}}} \\vspace{{0.08in}}\n\n", latex_escape(&parent_company)));
                
                for job in c2c_list {
                    let mut company_name = job.company.clone();
                    let item_cfg = if profile_items.is_empty() { None } else { profile_items.iter().find(|i| i.item_type == "job" && i.item_id == job.id) };
                    if let Some(cfg) = item_cfg {
                        if let Some(ref custom) = cfg.custom_name {
                            if !custom.is_empty() { company_name = custom.clone(); }
                        }
                    }
                    
                    let date_str = if job.hide_date { String::new() } else { job.date_range.clone() };

                    tex_content.push_str(&format!(
                        "\\noindent \\textbf{{Client: {}}} \\hfill {} \\\\\n\\textit{{{}}} \\vspace{{0.05in}}\n",
                        latex_escape(&company_name),
                        latex_escape(&date_str),
                        latex_escape(&job.role)
                    ));

                    tex_content.push_str("\\begin{itemize}[leftmargin=*,noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt]\n");
                    for bullet in &job.bullets {
                        tex_content.push_str(&format!("\\item {}\n", latex_escape(bullet)));
                    }
                    tex_content.push_str("\\end{itemize}\n\\vspace{0.1in}\n\n");
                }
                tex_content.push_str("\\vspace{0.1in}\n\n");
            }
        }
    }

    // Render Projects
    let profile_has_projects = profile_items.iter().any(|i| i.item_type == "project") || profile_items.is_empty();
    if profile_has_projects && !projects.is_empty() {
        tex_content.push_str("\\section*{Projects}\n");
        for proj in projects.iter().rev() {
            let item_cfg = if profile_items.is_empty() { None } else { profile_items.iter().find(|i| i.item_type == "project" && i.item_id == proj.id) };
            if profile_items.is_empty() || item_cfg.is_some() {
                let mut title_name = proj.title.clone();
                if let Some(cfg) = item_cfg {
                    if let Some(ref custom) = cfg.custom_name {
                        if !custom.is_empty() {
                            title_name = custom.clone();
                        }
                    }
                }

                tex_content.push_str(&format!(
                    "\\noindent \\textbf{{{}}} \\hfill {} \\vspace{{0.05in}}\n",
                    latex_escape(&title_name),
                    latex_escape(&proj.date_range)
                ));

                tex_content.push_str("\\begin{itemize}[leftmargin=*,noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt]\n");
                for bullet in &proj.bullets {
                    tex_content.push_str(&format!("\\item {}\n", latex_escape(bullet)));
                }
                tex_content.push_str("\\end{itemize}\n\\vspace{0.15in}\n\n");
            }
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

    let settings = crate::pages::landing::get_site_settings().await.unwrap_or_default();

    let mut bio = "Specializing in Enterprise Cloud Solutions, APEX, and Rust External Microservices. Dedicated to translating complex systems into immutable data flows.".to_string();
    let mut profile_items = Vec::new();

    if profile_id > 0 {
        let profile_row = sqlx::query("SELECT biography FROM resume_profiles WHERE id = $1")
            .bind(profile_id)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| -> ServerFnError { ServerFnError::ServerError("Profile not found".into()) })?;

        bio = profile_row.get("biography");

        let items_rows = sqlx::query("SELECT profile_id, item_type, item_id, custom_name FROM resume_profile_items WHERE profile_id = $1")
            .bind(profile_id).fetch_all(&state.pool).await?;

        profile_items = items_rows.into_iter().map(|row| ResumeProfileItem {
            profile_id: row.get("profile_id"),
            item_type: row.get("item_type"),
            item_id: row.get("item_id"),
            custom_name: row.get("custom_name"),
        }).collect();
    }

    let jobs = crate::pages::resume::get_jobs().await.unwrap_or_default();
    let projects = crate::pages::projects::get_projects().await.unwrap_or_default();
    
    let tex_content = generate_latex_string(&settings.site_title, &settings.hero_subtitle, &settings.hero_quote, &bio, &jobs, &projects, &profile_items);

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
        let bio = "Test bio";
        let jobs = vec![crate::pages::resume::JobRecord {
            id: 1,
            date_range: "2020-2022".into(),
            role: "Developer".into(),
            company: "Tech Corp".into(),
            bullets: vec!["Did things & stuff".into()],
            is_client_project: false,
            tags: vec![],
            hide_date: false,
        }];
        
        let tex = generate_latex_string("John", "Dev", "Quote", bio, &jobs, &[], &[]);
        
        assert!(tex.contains("Test bio"));
        assert!(tex.contains("Tech Corp"));
        assert!(tex.contains("Did things \\& stuff"));
        assert!(tex.contains("\\section*{Experience}"));
    }

    #[test]
    fn test_generate_latex_string_filters_and_masks() {
        let bio = "Test bio";
        let jobs = vec![
            crate::pages::resume::JobRecord {
                id: 1,
                date_range: "2020-2022".into(),
                role: "Developer".into(),
                company: "Tech Corp".into(),
                bullets: vec![],
                is_client_project: false,
                tags: vec![],
                hide_date: false,
            },
            crate::pages::resume::JobRecord {
                id: 2,
                date_range: "2018-2020".into(),
                role: "Engineer".into(),
                company: "Hidden Corp".into(),
                bullets: vec![],
                is_client_project: false,
                tags: vec![],
                hide_date: false,
            }
        ];
        
        let profile_items = vec![
            ResumeProfileItem {
                profile_id: 1,
                item_type: "job".into(),
                item_id: 2,
                custom_name: Some("Anonymous Financial Client".into()),
            }
        ];
        
        let tex = generate_latex_string("Jane", "Lead", "Hello", bio, &jobs, &[], &profile_items);
        
        assert!(!tex.contains("Tech Corp"));
        assert!(!tex.contains("Hidden Corp"));
        assert!(tex.contains("Anonymous Financial Client"));
    }
}
