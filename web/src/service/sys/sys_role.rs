use std::collections::HashMap;

use anyhow::{bail, Result};
use chrono::Local;
use models::dto::sys::response::sys_user::ResponseUser;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter};
use uuid::Uuid;

use common::db::get_db;
use entity::sys_role::{ActiveModel, Column as SysRoleColumn, Entity as SysRole};
use entity::sys_tenant::{Column as SysTenantColumn, Entity as SysTenant};
use models::dto::sys::request::sys_role::{AddRoleDto, SearchRoleDto, UpdateRoleDto};
use models::dto::sys::response::sys_role::ResponseRole;
use models::dto::sys::response::sys_tenant::ResponseTenant;
use models::dto::{handler_page, PageResult};

use crate::service::has_tenant;
use crate::service::sys::ADMIN_ID;

/// 根据角色编号查询角色数据
/// @param id 角色编号
pub async fn get_by_id(id: String) -> Result<Option<ResponseRole>> {
    let db = get_db().await;
    let result = SysRole::find_by_id(id.clone())
        // 保障删除字段为空
        .filter(SysRoleColumn::DeletedAt.is_null())
        .one(&db)
        .await?;
    match result {
        None => {
            bail!("编号：{}，数据不存在", id)
        }
        Some(data) => {
            let mut role: ResponseRole = data.clone().into();
            // 查询租户数据
            let tenant = SysTenant::find_by_id(data.tenant_id.unwrap())
                .one(&db)
                .await?;
            if tenant.is_some() {
                let tenant = tenant.unwrap();
                role.tenant = Some(tenant.into());
            }
            Ok(Some(role))
        }
    }
}

/// 根据角色编号删除角色数据
/// @param id 角色编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询角色信息
    let result = SysRole::find_by_id(id.clone()).one(&db).await?;
    if let Some(data) = result {
        // 查看角色是否为超级管理员
        if ADMIN_ID == data.id {
            bail!("超级管理员角色无法删除".to_string())
        } else {
            // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
            match force {
                true => {
                    // 删除角色
                    let delete_result = SysRole::delete_by_id(id).exec(&db).await?;
                    Ok(format!("{}", delete_result.rows_affected))
                }
                false => {
                    // 更新删除字段
                    let mut current: ActiveModel = data.into();
                    current.deleted_at = Set(Some(Local::now().naive_local()));
                    // 更新删除字段数据
                    let update_result = current.update(&db).await?;
                    Ok(update_result.id)
                }
            }
        }
    } else {
        bail!("该角色不存在".to_string())
    }
}

/// 添加角色
/// @param 角色对象
pub async fn add(data: AddRoleDto) -> Result<String> {
    let db = get_db().await;
    // 判断角色名是否为空
    if data.name.is_empty() {
        bail!("名称不能为空".to_string())
    }
    // 判断租户是否存在
    has_tenant(&data.tenant_id, &db).await?;

    let current: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(data.name),
        enabled: Set(0),
        tenant_id: Set(data.tenant_id),
        remark: Set(data.remark),
        description: Set(data.description),
        created_at: Set(Local::now().naive_local()),
        updated_at: NotSet,
        deleted_at: NotSet,
    };

    let add_role = current.insert(&db).await?;
    Ok(add_role.id)
}

/// 更新角色信息
/// @param update_role 待更新的角色对象
pub async fn update(id: String, update_role: UpdateRoleDto) -> Result<String> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("角色编号不能为空".to_string())
    }
    if ADMIN_ID == id {
        bail!("超级管理员角色无法更新".to_string())
    }
    // 查询角色信息
    let result = SysRole::find_by_id(id.clone()).one(&db).await?;
    // 更新角色信息
    if let Some(current) = result {
        let mut current: ActiveModel = current.into();
        // 名称是否为空
        if update_role.name.is_some() {
            current.name = Set(update_role.name.unwrap());
        }
        // 更新备注
        if update_role.remark.is_some() {
            current.remark = Set(Some(update_role.remark.unwrap()))
        }
        // 更新描述
        if update_role.description.is_some() {
            current.description = Set(Some(update_role.description.unwrap()))
        }
        // 更新时间
        current.updated_at = Set(Some(Local::now().naive_local()));
        // 更新数据
        let update_data = current.update(&db).await?;
        Ok(update_data.id)
    } else {
        bail!("角色数据不存在，无法更新".to_string())
    }
}

/// 更新状态
/// @param id role编号
/// @param status 状态，值为【0或1】，其他值无效
pub async fn update_status(id: String, status: i32) -> Result<String> {
    let db: sea_orm::DatabaseConnection = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("角色编号不能为空".to_string())
    }
    // 判断是否为超级管理员
    if ADMIN_ID == id {
        bail!("超级管理员无法变更状态".to_string())
    }
    // 查询角色信息
    let result = SysRole::find_by_id(id).one(&db).await?;
    // 更新用户名
    if let Some(current_data) = result {
        let mut current_data: ActiveModel = current_data.into();
        match status {
            0 | 1 => current_data.enabled = Set(status),
            _ => {
                bail!("角色状态值不合法，只能是【0或1】".to_string())
            }
        };
        // 更新时间
        current_data.updated_at = Set(Some(Local::now().naive_local()));
        // 更新状态
        let ok = current_data.update(&db).await?;
        Ok(ok.id)
    } else {
        bail!("角色数据不存在，无法更新".to_string())
    }
}


/// 查询角色列表
/// @param data 类型SearchRoleDto
pub async fn search(user: ResponseUser, data: SearchRoleDto) -> Result<PageResult<ResponseRole>> {
    let db = get_db().await;
    let mut select = SysRole::find();
    // 判断名称是否为空
    if data.name.is_some() {
        select = select.filter(SysRoleColumn::Name.contains(data.name.unwrap()))
    }
    // 判断状态是否为空
    if data.enabled.is_some() {
        select = select.filter(SysRoleColumn::Enabled.eq(data.enabled.unwrap()));
    }
    // 判断备注是否为空
    if data.remark.is_some() {
        select = select.filter(SysRoleColumn::Remark.contains(data.remark.unwrap()));
    }
    // 判断描述是否为空
    if data.description.is_some() {
        select = select.filter(SysRoleColumn::Description.contains(data.description.unwrap()));
    }
    // 判断租户是否为空
    if data.tenant_id.is_some() {
        let tenant_id = user.tenant;
        match tenant_id {
            Some(tenant) => select = select.filter(SysRoleColumn::TenantId.eq(tenant.id)),
            None => select = select.filter(SysRoleColumn::TenantId.eq(data.tenant_id.unwrap())),
        }
    } else if let Some(tenant) = user.tenant {
        select = select.filter(SysRoleColumn::TenantId.eq(tenant.id))
    }
    // 排除已删除角色
    select = select.filter(SysRoleColumn::DeletedAt.is_null());
    // 查询数据数量
    let total = select.clone().count(&db).await?;
    let (page_no, page_size) = handler_page(data.page);
    // 分页对象
    let paginate = select.paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;

    // 查询角色数据
    let list = paginate.fetch_page(page_no - 1).await?;

    let tenant_ids: Vec<String> = list
        .iter()
        .filter_map(|item| item.tenant_id.clone())
        .collect();

    let tenants = SysTenant::find()
        .filter(SysTenantColumn::Id.is_in(tenant_ids))
        .all(&db)
        .await?
        .iter()
        .map(|item| (item.id.clone(), item.clone().into()))
        .collect::<HashMap<String, ResponseTenant>>();

    let mut result_list = Vec::new();
    // 重组数据
    for role in list.into_iter() {
        let tenant_id = role.tenant_id.clone();
        let mut response_role: ResponseRole = role.into();
        if let Some(id) = tenant_id {
            let tenant = tenants.get(&id).and_then(|item|Some(item.clone()));
            response_role.tenant = tenant;
        }
        
        result_list.push(response_role);
    }
    let result = PageResult::new(result_list, total, pages, page_no);
    Ok(result)
}
