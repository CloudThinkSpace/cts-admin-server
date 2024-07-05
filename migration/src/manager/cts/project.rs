use sea_orm_migration::prelude::*;
use crate::TableOperation;

#[derive(DeriveIden)]
pub enum Project {
    Table,
    // 编号
    Id,
    // 项目名称
    Name,
    // 项目编号
    Code,
    // 表单编号
    FormTemplateId,
    // 数据变化
    DataTableName,
    // 数据条数
    Total,
    // 项目类型
    Type,
    // 状态
    Status,
    // 描述
    Description,
    // 备注
    Remark,
    // 创建时间
    CreatedAt,
    // 更新时间
    UpdatedAt,
    // 删除时间
    DeletedAt,
}

impl TableOperation for  Project {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Project::Name).string().not_null())
                    .col(ColumnDef::new(Project::Code).string().not_null())
                    .col(ColumnDef::new(Project::FormTemplateId).string().not_null())
                    .col(ColumnDef::new(Project::DataTableName).string().not_null())
                    .col(ColumnDef::new(Project::Total).integer().not_null().default(0))
                    .col(ColumnDef::new(Project::Type).integer().not_null().default(0))
                    .col(ColumnDef::new(Project::Status).integer().not_null().default(0))
                    .col(ColumnDef::new(Project::Description).string())
                    .col(ColumnDef::new(Project::Remark).string())
                    .col(ColumnDef::new(Project::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Project::UpdatedAt).timestamp())
                    .col(ColumnDef::new(Project::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn create_index(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        // 创建唯一键
        manager.create_index(Index::create()
            .table(Project::Table)
            .if_not_exists()
            .name("unique_project_code")
            .col(Project::Code)
            .unique()
            .to_owned()
        ).await?;
        Ok(())
    }

    async fn drop_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Project::Table).if_exists().to_owned()).await
    }

    async fn insert_data(&self, _manager: &SchemaManager<'_>) -> Result<(), DbErr> {

        Ok(())
    }
}