use sea_orm::{DbErr};
use sea_orm_migration::SchemaManager;
use crate::sys::sys_api::SysApi;
use crate::sys::sys_menu::SysMenu;
use crate::sys::sys_role::SysRole;
use crate::sys::sys_role_menu::SysRoleMenu;
use crate::sys::sys_tenant::SysTenant;
use crate::sys::sys_user::SysUser;

mod sys_user;
mod sys_role;
mod sys_tenant;
mod sys_menu;
mod sys_api;
mod sys_role_menu;

trait TableOperation {
    async fn create_table(manager: &SchemaManager<'_>) -> Result<(), DbErr>;
    async fn create_index(manager: &SchemaManager<'_>) -> Result<(), DbErr>;
    async fn drop_table(manager: &SchemaManager<'_>) -> Result<(), DbErr>;
    async fn insert_data(manager: &SchemaManager<'_>) -> Result<(), DbErr>;
}

pub async fn create_tables(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    SysTenant::create_table(manager).await?;
    SysUser::create_table(manager).await?;
    SysRole::create_table(manager).await?;
    SysMenu::create_table(manager).await?;
    SysApi::create_table(manager).await?;
    SysRoleMenu::create_table(manager).await?;
    Ok(())
}

pub async fn create_indices(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    SysUser::create_index(manager).await?;
    SysRoleMenu::create_index(manager).await?;
    Ok(())
}

pub async fn drop_tables(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    SysRoleMenu::drop_table(manager).await?;
    SysUser::drop_table(manager).await?;
    SysRole::drop_table(manager).await?;
    SysMenu::drop_table(manager).await?;
    SysApi::drop_table(manager).await?;
    SysTenant::drop_table(manager).await?;
    Ok(())
}

pub async fn insert_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    SysRole::insert_data(manager).await?;
    SysUser::insert_data(manager).await?;
    SysMenu::insert_data(manager).await?;
    SysTenant::insert_data(manager).await?;
    SysRoleMenu::insert_data(manager).await?;
    Ok(())
}