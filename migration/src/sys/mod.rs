use sea_orm::{DbErr};
use sea_orm_migration::SchemaManager;
use crate::sys::sys_api::SysApi;
use crate::sys::sys_menu::SysMenu;
use crate::sys::sys_role::SysRole;
use crate::sys::sys_tenant::SysTenant;
use crate::sys::sys_user::SysUser;

mod sys_user;
mod sys_role;
mod sys_tenant;
mod sys_menu;
mod sys_api;

pub async fn create_tables(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    SysTenant::create_table(manager).await?;
    SysUser::create_table(manager).await?;
    SysRole::create_table(manager).await?;
    SysMenu::create_table(manager).await?;
    SysApi::create_table(manager).await?;
    Ok(())
}

pub async fn drop_tables(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    SysUser::drop_table(manager).await?;
    SysRole::drop_table(manager).await?;
    SysMenu::drop_table(manager).await?;
    SysApi::drop_table(manager).await?;
    SysTenant::drop_table(manager).await?;
    Ok(())
}

pub async fn insert_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    SysUser::insert(manager).await?;
    SysRole::insert(manager).await?;
    Ok(())
}