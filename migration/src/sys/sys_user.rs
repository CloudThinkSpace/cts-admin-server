use sea_orm_migration::prelude::*;
use crate::sys::sys_tenant::SysTenant;

#[derive(DeriveIden)]
pub enum SysUser {
    Table,
    // 编号
    Id,
    // 用户名称
    Username,
    // 用户昵称
    Nickname,
    // 密码
    Password,
    // 电话
    Phone,
    // 邮箱
    Email,
    // 状态
    Status,
    // 描述
    Description,
    // 备注
    Remark,
    // 用户头像
    Avatar,
    // 租户编号
    TenantId,
    // 创建时间
    CreatedAt,
    // 更新时间
    UpdatedAt,
    // 删除时间
    DeletedAt,
}

impl SysUser {
    pub async fn create_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysUser::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysUser::Username).string().not_null())
                    .col(ColumnDef::new(SysUser::Nickname).string().not_null())
                    .col(ColumnDef::new(SysUser::Password).string().not_null())
                    .col(ColumnDef::new(SysUser::Phone).string().not_null())
                    .col(ColumnDef::new(SysUser::Email).string())
                    .col(ColumnDef::new(SysUser::Status).integer().not_null().default(0))
                    .col(ColumnDef::new(SysUser::Remark).string())
                    .col(ColumnDef::new(SysUser::Description).string())
                    .col(ColumnDef::new(SysUser::Avatar).string_len(300))
                    .col(ColumnDef::new(SysUser::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(SysUser::UpdatedAt).timestamp())
                    .col(ColumnDef::new(SysUser::DeletedAt).timestamp())
                    .col(ColumnDef::new(SysUser::TenantId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_tenant_id")
                            .from(SysUser::Table, SysUser::TenantId)
                            .to(SysTenant::Table, SysTenant::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    pub async fn drop_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysUser::Table).to_owned()).await
    }
}