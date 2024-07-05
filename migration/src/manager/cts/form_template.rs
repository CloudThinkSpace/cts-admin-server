use sea_orm_migration::prelude::*;
use crate::TableOperation;

#[derive(DeriveIden)]
pub enum FormTemplate {
    Table,
    // 编号
    Id,
    // api名称
    Name,
    // 标题
    Title,
    // 表单内容
    Content,
    // api请求方式
    Version,
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

impl TableOperation for  FormTemplate {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FormTemplate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FormTemplate::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FormTemplate::Name).string().not_null())
                    .col(ColumnDef::new(FormTemplate::Title).string().not_null())
                    .col(ColumnDef::new(FormTemplate::Content).string())
                    .col(ColumnDef::new(FormTemplate::Version).string().not_null())
                    .col(ColumnDef::new(FormTemplate::Description).string())
                    .col(ColumnDef::new(FormTemplate::Remark).string())
                    .col(ColumnDef::new(FormTemplate::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(FormTemplate::UpdatedAt).timestamp())
                    .col(ColumnDef::new(FormTemplate::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn create_index(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        // 创建唯一键
        manager.create_index(Index::create()
            .table(FormTemplate::Table)
            .if_not_exists()
            .name("unique_form-template_name")
            .col(FormTemplate::Name)
            .unique()
            .to_owned()
        ).await?;
        Ok(())
    }

    async fn drop_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FormTemplate::Table).if_exists().to_owned()).await
    }

    async fn insert_data(&self, _manager: &SchemaManager<'_>) -> Result<(), DbErr> {

        Ok(())
    }
}