use sea_orm_migration::prelude::*;
use crate::sys::sys_tenant::SysTenant;
use crate::sys::sys_user::SysUser;

#[derive(DeriveIden)]
pub enum SysRole {
    Table,
    // 编号
    Id,
    // 角色名称
    Name,
    // 是否启用，0启用，1不启用
    Enabled,
    // 租户编号
    TenantId,
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

impl SysRole {
    pub async fn create_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysRole::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysRole::Name).string().not_null())
                    .col(ColumnDef::new(SysRole::Enabled).integer().not_null().default(0))
                    .col(ColumnDef::new(SysRole::TenantId).string().not_null())
                    .col(ColumnDef::new(SysRole::Description).string())
                    .col(ColumnDef::new(SysRole::Remark).string())
                    .col(ColumnDef::new(SysRole::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(SysRole::UpdatedAt).timestamp())
                    .col(ColumnDef::new(SysRole::DeletedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_tenant_id")
                            .from(SysUser::Table, SysUser::TenantId)
                            .to(SysTenant::Table, SysTenant::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    pub async fn drop_table(manager: &SchemaManager<'_>)-> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysRole::Table).to_owned()).await
    }
}