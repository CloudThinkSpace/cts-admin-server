use chrono::Local;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use common::md5;
use crate::manager::sys::sys_role::SysRole;
use crate::TableOperation;

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
    // 状态,是否可用，0可用，1停用
    Status,
    // 描述
    Description,
    // 备注
    Remark,
    // 用户头像
    Avatar,
    // 租户编号
    TenantId,
    // 角色编号
    RoleId,
    // 创建时间
    CreatedAt,
    // 更新时间
    UpdatedAt,
    // 删除时间
    DeletedAt,
}
impl TableOperation for SysUser {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(SysUser::Avatar).string())
                    .col(ColumnDef::new(SysUser::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(SysUser::UpdatedAt).timestamp())
                    .col(ColumnDef::new(SysUser::DeletedAt).timestamp())
                    .col(ColumnDef::new(SysUser::TenantId).string())
                    .col(ColumnDef::new(SysUser::RoleId).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn create_index(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {

        // 创建唯一键
        manager.create_index(Index::create()
            .table(SysUser::Table)
            .if_not_exists()
            .name("unique_user_username")
            .col(SysUser::Username)
            .unique()
            .to_owned()
        ).await?;
        // 创建 角色外键
        manager.create_foreign_key(
            ForeignKey::create()
                .name("Fk_user_role_id")
                .from(SysUser::Table, SysUser::RoleId)
                .to(SysRole::Table, SysRole::Id)
                .to_owned()
        ).await

    }

    async fn drop_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysUser::Table).if_exists().to_owned()).await?;
        Ok(())
    }

    async fn insert_data(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {

        let db = manager.get_connection();
        // 生成时间戳
        let now = Local::now().naive_local();

        let stmt_user = Statement::from_sql_and_values(
            manager.get_database_backend(),
            "
        INSERT INTO sys_user
        (id, username, nickname, phone, email, password,role_id, created_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
        ",
            [
                "1".into(),
                "admin".into(),
                "超级管理员".into(),
                "18911797115".into(),
                "tanghy@cloudthink.space".into(),
                md5::generate_md5("123456".to_string()).into(),
                "1".into(),
                now.into()
            ]
        );
        db.execute(stmt_user).await?;
        Ok(())
    }
}