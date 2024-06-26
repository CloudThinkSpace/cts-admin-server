use sea_orm::DbErr;
use sea_orm_migration::SchemaManager;

use crate::{ColumnDef, DeriveIden, ForeignKey, Table};
use crate::sys::sys_menu::SysMenu;
use crate::sys::sys_role::SysRole;
use crate::sys::TableOperation;

#[derive(DeriveIden)]
pub enum SysRoleMenu {
    Table,
    Id,
    // 角色Id
    RoleId,
    // 菜单Id
    MenuId,
}

impl TableOperation for SysRoleMenu {
    async fn create_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysRoleMenu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysRoleMenu::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysRoleMenu::RoleId).string().not_null())
                    .col(ColumnDef::new(SysRoleMenu::MenuId).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn create_index(manager: &SchemaManager<'_>) -> Result<(), DbErr> {

        // 创建 角色外键
        manager.create_foreign_key(
            ForeignKey::create()
                .name("Fk_role-menu_role_id")
                .from(SysRoleMenu::Table, SysRoleMenu::RoleId)
                .to(SysRole::Table, SysRole::Id)
                .to_owned()
        ).await?;

        // 创建 角色外键
        manager.create_foreign_key(
            ForeignKey::create()
                .name("Fk_role-menu_menu_id")
                .from(SysRoleMenu::Table, SysRoleMenu::MenuId)
                .to(SysMenu::Table, SysMenu::Id)
                .to_owned()
        ).await

    }

    async fn drop_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysRoleMenu::Table).if_exists().to_owned()).await?;
        Ok(())
    }

    async fn insert_data(_manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        Ok(())
    }
}