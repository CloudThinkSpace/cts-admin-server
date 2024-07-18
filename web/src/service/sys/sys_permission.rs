use anyhow::{Ok, Result};
use async_recursion::async_recursion;
use common::db::get_db;
use entity::sys_api::{Column as SysApiColumn, Entity as SysApi};
use entity::sys_menu::{Column as SysMenuColumn, Entity as SysMenu};

use entity::sys_role_api::{
    ActiveModel as ActiveSysRoleApi, Column as SysRoleApiColumn, Entity as SysRoleApi,
    Relation as SysRoleApiRelation,
};
use entity::sys_role_menu::{
    ActiveModel as ActiveSysRoleMenu, Column as SysRoleMenuColumn, Entity as SysRoleMenu,
    Relation as SysRoleMenuRelation,
};

use models::dto::sys::request::sys_permission::{RoleApiDto, RoleMenuDto};
use models::dto::sys::response::sys_menu::ResponseMenu;
use sea_orm::sea_query::{Expr, IntoCondition};
use sea_orm::{
    ColumnTrait, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
    TransactionTrait,
};
use uuid::Uuid;

use super::SYSTEM_PARENT_MENU_ID;
/// 为角色授权菜单
///     
pub async fn set_menu(menus_role: RoleMenuDto) -> Result<String> {
    let db = get_db().await;
    // 获取事务对象
    let tx = db.begin().await?;

    let role_id = menus_role.role_id;
    let menu_ids = menus_role.menu_ids;
    // 删除已有角色授权的菜单
    let _delete_result = SysRoleMenu::delete_many()
        .filter(SysRoleMenuColumn::RoleId.eq(role_id.clone()))
        .exec(&tx)
        .await?;
    // 插入新授权列表
    // 组织数据
    let mut list = Vec::new();
    for id in menu_ids {
        let data = ActiveSysRoleMenu {
            id: Set(Uuid::new_v4().to_string()),
            role_id: Set(role_id.clone()),
            menu_id: Set(id),
        };
        list.push(data);
    }
    // 插入数据
    let _insert_result = SysRoleMenu::insert_many(list).exec(&tx).await?;
    // 提交数据
    tx.commit().await?;
    Ok("授权菜单成功".to_string())
}

/// 为角色授权api
///     
pub async fn set_api(apis_role: RoleApiDto) -> Result<String> {
    let db = get_db().await;
    // 获取事务对象
    let tx = db.begin().await?;

    let role_id = apis_role.role_id;
    let menu_ids = apis_role.api_ids;
    // 删除已有角色授权的api
    let _delete_result = SysRoleApi::delete_many()
        .filter(SysRoleApiColumn::RoleId.eq(role_id.clone()))
        .exec(&tx)
        .await?;
    // 插入新授权列表
    // 组织数据
    let mut list = Vec::new();
    for id in menu_ids {
        let data = ActiveSysRoleApi {
            id: Set(Uuid::new_v4().to_string()),
            role_id: Set(role_id.clone()),
            api_id: Set(id),
        };
        list.push(data);
    }
    // 插入数据
    let _insert_result = SysRoleApi::insert_many(list).exec(&tx).await?;
    // 提交数据
    tx.commit().await?;
    Ok("授权Api成功".to_string())
}

pub async fn search_api(role_id: String) -> Result<Vec<String>> {
    let db = get_db().await;
    let result: Vec<String> = SysApi::find()
        .join(
            JoinType::LeftJoin,
            SysRoleApiRelation::SysApi
                .def()
                .rev()
                .on_condition(move |_left, right| {
                    Expr::col((right, SysRoleApiColumn::RoleId))
                        .eq(role_id.clone())
                        .into_condition()
                }),
        )
        .order_by_asc(SysApiColumn::ApiGroup)
        .all(&db)
        .await?
        .into_iter()
        .map(|model| model.id)
        .collect();
    Ok(result)
}

pub async fn search_menu(role_id: String) -> Result<Vec<ResponseMenu>> {
    let db = get_db().await;
    let result: Vec<ResponseMenu> = SysMenu::find()
        .join(
            JoinType::LeftJoin,
            SysRoleMenuRelation::SysMenu
                .def()
                .rev()
                .on_condition(move |_left, right| {
                    Expr::col((right, SysRoleMenuColumn::RoleId))
                        .eq(role_id.clone())
                        .into_condition()
                }),
        )
        .order_by_asc(SysMenuColumn::Sort)
        .all(&db)
        .await?
        .into_iter()
        .map(|model| model.into())
        .collect();

    let root_nodes = root_tree(&result).await;
    let data = child_tree(root_nodes, &result).await;
    Ok(data)
}

async fn root_tree(nodes: &[ResponseMenu]) -> Vec<ResponseMenu> {
    nodes
        .iter()
        .filter(|item| item.parent_id == SYSTEM_PARENT_MENU_ID)
        .cloned()
        .collect()
}

#[async_recursion]
async fn child_tree(mut roots: Vec<ResponseMenu>, nodes: &[ResponseMenu]) -> Vec<ResponseMenu> {
    for root in roots.iter_mut() {
        let data: Vec<ResponseMenu> = nodes
            .iter()
            .filter(|item| item.parent_id == root.id)
            .cloned()
            .collect();
        let data = child_tree(data.clone(), nodes).await;
        root.children = Some(data);
    }
    return roots;
}
