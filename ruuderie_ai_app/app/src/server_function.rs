use leptos::*;
use entity::{article, user, comment};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use {
    entity::sea_orm::{EntityTrait, ActiveModelTrait, ActiveValue::Set, DbErr},
    entity::db::DB,
};

// Custom error type
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Other(String),
}

// Helper function to convert AppError to ServerFnError
fn app_err_to_server_err(err: AppError) -> ServerFnError {
    ServerFnError::ServerError(err.to_string())
}

#[server(AddArticle, "/api")]
pub async fn add_article(title: String, content: String, author_id: Uuid) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = use_context::<DB>().expect("DB missing");
        let conn = db.conn();
        let article = article::ActiveModel {
            title: Set(title),
            content: Set(content),
            author_id: Set(author_id),
            ..Default::default()
        };
        article.insert(conn).await.map_err(AppError::from).map_err(app_err_to_server_err)?;
        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    Ok(())
}

#[server(GetArticle, "/api")]
pub async fn get_article(id: Uuid) -> Result<article::Model, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = use_context::<DB>().expect("DB missing");
        let conn = db.conn();
        let article = article::Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(AppError::from)
            .map_err(app_err_to_server_err)?;
        
        article.ok_or_else(|| app_err_to_server_err(AppError::NotFound("Article not found".to_string())))
    }

    #[cfg(not(feature = "ssr"))]
    Err(ServerFnError::ServerError("Not implemented".to_string()))
}

#[server(UpdateArticle, "/api")]
pub async fn update_article(id: Uuid, title: Option<String>, content: Option<String>) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = use_context::<DB>().expect("DB missing");
        let conn = db.conn();
        let article = article::Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(AppError::from)
            .map_err(app_err_to_server_err)?
            .ok_or_else(|| app_err_to_server_err(AppError::NotFound("Article not found".to_string())))?;

        let mut article: article::ActiveModel = article.into();
        
        if let Some(title) = title {
            article.title = Set(title);
        }
        if let Some(content) = content {
            article.content = Set(content);
        }
        
        article.update(conn).await.map_err(AppError::from).map_err(app_err_to_server_err)?;
        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    Ok(())
}

#[server(DeleteArticle, "/api")]
pub async fn delete_article(id: Uuid) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = use_context::<DB>().expect("DB missing");
        let conn = db.conn();
        article::Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map_err(AppError::from)
            .map_err(app_err_to_server_err)?;
        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    Ok(())
}

#[server(CreateUser, "/api")]
pub async fn create_user(username: String, email: String, password: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = use_context::<DB>().expect("DB missing");
        let conn = db.conn();
        let user = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password_hash: Set(password), // Note: In a real application, you should hash the password before storing
            ..Default::default()
        };
        user.insert(conn).await.map_err(AppError::from).map_err(app_err_to_server_err)?;
        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    Ok(())
}

#[server(GetUser, "/api")]
pub async fn get_user(id: Uuid) -> Result<user::Model, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = use_context::<DB>().expect("DB missing");
        let conn = db.conn();
        let user = user::Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(AppError::from)
            .map_err(app_err_to_server_err)?;
        
        user.ok_or_else(|| app_err_to_server_err(AppError::NotFound("User not found".to_string())))
    }

    #[cfg(not(feature = "ssr"))]
    Err(ServerFnError::ServerError("Not implemented".to_string()))
}

#[server(CreateComment, "/api")]
pub async fn create_comment(content: String, article_id: Uuid, user_id: Uuid) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = use_context::<DB>().expect("DB missing");
        let conn = db.conn();
        let comment = comment::ActiveModel {
            content: Set(content),
            article_id: Set(article_id),
            user_id: Set(user_id),
            ..Default::default()
        };
        comment.insert(conn).await.map_err(AppError::from).map_err(app_err_to_server_err)?;
        Ok(())
    }

    #[cfg(not(feature = "ssr"))]
    Ok(())
}