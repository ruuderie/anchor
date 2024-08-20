use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::Username).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Users::UserType).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Create categories table
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Categories::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(ColumnDef::new(Categories::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Categories::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // Create articles table
        manager
            .create_table(
                Table::create()
                    .table(Articles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Articles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Articles::Title).string().not_null())
                    .col(ColumnDef::new(Articles::Content).string().not_null())
                    .col(ColumnDef::new(Articles::AuthorId).uuid().not_null())
                    .col(ColumnDef::new(Articles::CategoryId).uuid().not_null())
                    .col(ColumnDef::new(Articles::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Articles::UpdatedAt).date_time().not_null())
                    .foreign_key(ForeignKey::create().from(Articles::Table, Articles::AuthorId).to(Users::Table, Users::Id))
                    .foreign_key(ForeignKey::create().from(Articles::Table, Articles::CategoryId).to(Categories::Table, Categories::Id))
                    .to_owned(),
            )
            .await?;

        // Create article_categories table
        manager
            .create_table(
                Table::create()
                    .table(ArticleCategories::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ArticleCategories::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ArticleCategories::CategoryId).uuid().not_null())
                    .col(ColumnDef::new(ArticleCategories::ArticleId).uuid().not_null())
                    .col(ColumnDef::new(ArticleCategories::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(ArticleCategories::UpdatedAt).date_time().not_null())
                    .foreign_key(ForeignKey::create().from(ArticleCategories::Table, ArticleCategories::CategoryId).to(Categories::Table, Categories::Id))
                    .foreign_key(ForeignKey::create().from(ArticleCategories::Table, ArticleCategories::ArticleId).to(Articles::Table, Articles::Id))
                    .to_owned(),
            )
            .await?;

        // Create comments table
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comments::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Comments::ArticleId).uuid().not_null())
                    .col(ColumnDef::new(Comments::UserId).uuid().not_null())
                    .col(ColumnDef::new(Comments::Content).string().not_null())
                    .col(ColumnDef::new(Comments::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Comments::UpdatedAt).date_time().not_null())
                    .foreign_key(ForeignKey::create().from(Comments::Table, Comments::ArticleId).to(Articles::Table, Articles::Id))
                    .foreign_key(ForeignKey::create().from(Comments::Table, Comments::UserId).to(Users::Table, Users::Id))
                    .to_owned(),
            )
            .await?;

        // Create auth_tokens table
        manager
            .create_table(
                Table::create()
                    .table(AuthTokens::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AuthTokens::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AuthTokens::UserId).uuid().not_null())
                    .col(ColumnDef::new(AuthTokens::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(AuthTokens::ExpiresAt).date_time().not_null())
                    .col(ColumnDef::new(AuthTokens::Purpose).string().null())
                    .col(ColumnDef::new(AuthTokens::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(AuthTokens::UsedAt).date_time().null())
                    .foreign_key(ForeignKey::create().from(AuthTokens::Table, AuthTokens::UserId).to(Users::Table, Users::Id))
                    .to_owned(),
            )
            .await?;

        // Create landing_pages table
        manager
            .create_table(
                Table::create()
                    .table(LandingPages::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(LandingPages::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(LandingPages::Heading).string().not_null())
                    .col(ColumnDef::new(LandingPages::Subheading).string().not_null())
                    .col(ColumnDef::new(LandingPages::CallToAction).string().not_null())
                    .col(ColumnDef::new(LandingPages::VideoUrl).string().not_null())
                    .col(ColumnDef::new(LandingPages::CompanyLogos).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(LandingPages::Benefits).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(LandingPages::HowItWorks).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(LandingPages::Testimonials).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(LandingPages::FAQ).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(LandingPages::Footer).string().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LandingPages::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AuthTokens::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ArticleCategories::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Articles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
    UserType,
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Articles {
    Table,
    Id,
    Title,
    Content,
    AuthorId,
    CategoryId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ArticleCategories {
    Table,
    Id,
    CategoryId,
    ArticleId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    ArticleId,
    UserId,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AuthTokens {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
    Purpose,
    CreatedAt,
    UsedAt,
}

#[derive(DeriveIden)]
enum LandingPages {
    Table,
    Id,
    Heading,
    Subheading,
    CallToAction,
    VideoUrl,
    CompanyLogos,
    Benefits,
    HowItWorks,
    Testimonials,
    FAQ,
    Footer,
}