use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建项目图层表
        manager
            .create_table(
                Table::create()
                    .table(ProjectLayer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectLayer::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                     .col(ColumnDef::new(ProjectLayer::ProjectId).string().not_null())
                    .col(ColumnDef::new(ProjectLayer::Name).string().not_null())
                    .col(ColumnDef::new(ProjectLayer::Type).string().not_null())
                    .col(ColumnDef::new(ProjectLayer::Url).string().not_null())
                    .col(ColumnDef::new(ProjectLayer::Format).string().not_null())
                    .col(ColumnDef::new(ProjectLayer::Checked).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(ProjectLayer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProjectLayer {
    Table,
    Id,
    ProjectId,
    Name,
    Type,
    Url,
    Format,
    Checked,
}
