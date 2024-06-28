use sea_orm_migration::prelude::*;

use crate::TableOperation;

#[derive(DeriveIden)]
pub enum SysApi {
    Table,
    // 编号
    Id,
    // api名称
    Name,
    // api请求url
    ApiPath,
    // api分组
    ApiGroup,
    // api请求方式
    ApiMethod,
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

impl TableOperation for SysApi {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysApi::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysApi::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysApi::Name).string().not_null())
                    .col(ColumnDef::new(SysApi::ApiPath).string().not_null())
                    .col(ColumnDef::new(SysApi::ApiGroup).string().not_null())
                    .col(ColumnDef::new(SysApi::ApiMethod).string().not_null())
                    .col(ColumnDef::new(SysApi::Description).string())
                    .col(ColumnDef::new(SysApi::Remark).string())
                    .col(ColumnDef::new(SysApi::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(SysApi::UpdatedAt).timestamp())
                    .col(ColumnDef::new(SysApi::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn create_index(&self, _manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        Ok(())
    }

    async fn drop_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysApi::Table).if_exists().to_owned()).await
    }

    async fn insert_data(&self, _manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        Ok(())
    }
}