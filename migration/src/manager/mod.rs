use sea_orm::DbErr;
use sea_orm_migration::SchemaManager;
use crate::manager::cts::form_template::FormTemplate;
use crate::manager::cts::project::Project;
use crate::manager::sys::sys_api::SysApi;
use crate::manager::sys::sys_menu::SysMenu;
use crate::manager::sys::sys_role::SysRole;
use crate::manager::sys::sys_role_menu::SysRoleMenu;
use crate::manager::sys::sys_tenant::SysTenant;
use crate::manager::sys::sys_user::SysUser;
use crate::TableOperation;

pub mod cts;
pub mod sys;


pub async fn create_tables(manager: &SchemaManager<'_>) -> Result<(), DbErr>
{
    SysTenant::Table.create_table(manager).await?;
    SysMenu::Table.create_table(manager).await?;
    SysUser::Table.create_table(manager).await?;
    SysRole::Table.create_table(manager).await?;
    SysApi::Table.create_table(manager).await?;
    SysRoleMenu::Table.create_table(manager).await?;

    FormTemplate::Table.create_table(manager).await?;
    Project::Table.create_table(manager).await?;

    Ok(())
}

pub async fn create_indices(manager: &SchemaManager<'_>) -> Result<(), DbErr>
{
    SysTenant::Table.create_index(manager).await?;
    SysUser::Table.create_index(manager).await?;
    SysRole::Table.create_index(manager).await?;
    SysApi::Table.create_index(manager).await?;
    SysMenu::Table.create_index(manager).await?;
    SysRoleMenu::Table.create_index(manager).await?;

    FormTemplate::Table.create_index(manager).await?;
    Project::Table.create_index(manager).await?;
    Ok(())
}

pub async fn drop_tables(manager: &SchemaManager<'_>) -> Result<(), DbErr>
{
    SysRoleMenu::Table.drop_table(manager).await?;
    SysTenant::Table.drop_table(manager).await?;
    SysUser::Table.drop_table(manager).await?;
    SysRole::Table.drop_table(manager).await?;
    SysApi::Table.drop_table(manager).await?;
    SysMenu::Table.drop_table(manager).await?;

    FormTemplate::Table.drop_table(manager).await?;
    Project::Table.drop_table(manager).await?;
    Ok(())
}

pub async fn insert_data(manager: &SchemaManager<'_>) -> Result<(), DbErr>
{
    SysTenant::Table.insert_data(manager).await?;
    SysRole::Table.insert_data(manager).await?;
    SysUser::Table.insert_data(manager).await?;
    SysApi::Table.insert_data(manager).await?;
    SysMenu::Table.insert_data(manager).await?;
    SysRoleMenu::Table.insert_data(manager).await?;

    FormTemplate::Table.insert_data(manager).await?;
    Project::Table.insert_data(manager).await?;
    Ok(())
}
