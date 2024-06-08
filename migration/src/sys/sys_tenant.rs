use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum SysTenant {
    Table,
    // 编号
    Id,
    // 租户名称
    Name,
    // 描述
    Description,
    // 备注
    Remark,
    // 租户类型，免费租户0，付费租户1
    Type,
    // 启用0、禁用1
    Enabled,
    // 创建时间
    CreatedAt,
    // 更新时间
    UpdatedAt,
    // 删除时间
    DeletedAt,
}

impl SysTenant {
    pub async fn create_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysTenant::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysTenant::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysTenant::Name).string().not_null())
                    .col(ColumnDef::new(SysTenant::Enabled).integer().not_null().default(0))
                    .col(ColumnDef::new(SysTenant::Type).integer().not_null().default(0))
                    .col(ColumnDef::new(SysTenant::Description).string())
                    .col(ColumnDef::new(SysTenant::Remark).string())
                    .col(ColumnDef::new(SysTenant::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(SysTenant::UpdatedAt).timestamp())
                    .col(ColumnDef::new(SysTenant::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    pub async fn drop_table(manager: &SchemaManager<'_>)-> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysTenant::Table).to_owned()).await
    }
}