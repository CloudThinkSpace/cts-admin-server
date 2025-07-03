use std::collections::HashMap;

use anyhow::{bail, Result};
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use common::db::get_db;
use common::md5::generate_md5;
use entity::sys_role::{Column as SysRoleColumn, Entity as SysRole};
use entity::sys_tenant::{Column as SysTenantColumn, Entity as SysTenant};
use entity::sys_user::{ActiveModel, Column as SysUserColumn, Entity as SysUser};
use models::dto::sys::request::sys_user::{AddUserDto, SearchUserDto, UpdateUserDto};
use models::dto::sys::response::sys_role::ResponseRole;
use models::dto::sys::response::sys_tenant::ResponseTenant;
use models::dto::sys::response::sys_user::ResponseUser;
use models::dto::{handler_page, PageResult};

use crate::service::has_tenant;
use crate::service::sys::ADMIN_ID;

/// 根据用户编号查询用户数据
/// @param id 用户编号
pub async fn get_by_id(id: String) -> Result<Option<ResponseUser>> {
    let db = get_db().await;
    let result = SysUser::find_by_id(id)
        // 保障删除字段为空
        .filter(SysUserColumn::DeletedAt.is_null())
        .one(&db)
        .await?;
    match result {
        None => Ok(None),
        Some(data) => {
            let mut user: ResponseUser = data.clone().into();
            // 查询用户和租户
            if data.tenant_id.is_some() {
                let tenant = SysTenant::find_by_id(data.tenant_id.unwrap())
                    .one(&db)
                    .await?;
                user.tenant = Some(tenant.unwrap().into());
            } else {
                let role = SysRole::find_by_id(data.role_id).one(&db).await?;
                if role.is_some() {
                    user.role = Some(role.unwrap().into());
                }
            }
            Ok(Some(user))
        }
    }
}

/// 根据用户编号删除用户数据
/// @param id 用户编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询用户信息
    let result = SysUser::find_by_id(id.clone()).one(&db).await?;
    if let Some(data) = result {
        // 查看用户是否为超级管理员
        if ADMIN_ID == data.id {
            bail!("超级管理员无法删除")
        } else {
            // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
            match force {
                true => {
                    // 删除用户
                    let _delete_result = SysUser::delete_by_id(id).exec(&db).await?;
                    Ok("删除成功".to_string())
                }
                false => {
                    // 更新删除字段
                    let mut current_user: ActiveModel = data.into();
                    current_user.deleted_at = Set(Some(Local::now().naive_local()));
                    // 更新删除字段数据
                    let _update_result = current_user.update(&db).await?;
                    Ok("删除用户成功".to_string())
                }
            }
        }
    } else {
        bail!("该用户不存在")
    }
}

/// 添加用户
/// @param 用户对象
pub async fn add(data: AddUserDto) -> Result<String> {
    let db = get_db().await;

    // 判断用户名是否为空
    if data.username.is_empty() {
        bail!("用户名不能为空".to_string())
    }
    // 用户名查询数据
    let user = SysUser::find()
        .filter(SysUserColumn::Username.eq(data.username.clone()))
        .one(&db)
        .await?;
    // 判断用户是否存在
    if user.is_some() {
        bail!("用户名不能重复，该用户已经存在".to_string())
    }
    // 判断租户是否存在
    has_tenant(&data.tenant_id, &db).await?;

    let current_user: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        username: Set(data.username),
        nickname: Set(data.nickname),
        password: Set(generate_md5(data.password)),
        phone: Set(data.phone),
        tenant_id: Set(data.tenant_id),
        role_id: Set(data.role_id),
        email: Set(data.email),
        remark: Set(data.remark),
        avatar: Set(data.avatar),
        description: Set(data.description),
        created_at: Set(Local::now().naive_local()),
        updated_at: NotSet,
        deleted_at: NotSet,
        ..Default::default()
    };
    let add_user = current_user.insert(&db).await?;
    Ok(add_user.id)
}

/// 更新用户信息
/// @param update_user 待更新的用户对象
pub async fn update(id: String, update_user: UpdateUserDto) -> Result<String> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("用户编号不能为空".to_string())
    }
    if ADMIN_ID == id {
        bail!("超级用户无法更新".to_string())
    }
    // 查询用户信息
    let result = SysUser::find_by_id(id.clone()).one(&db).await?;
    // todo 角色是否存在
    // 更新用户名
    if let Some(current_user) = result {
        let mut current_user: ActiveModel = current_user.into();
        // 昵称是否为空
        if update_user.nickname.is_some() {
            current_user.nickname = Set(update_user.nickname.unwrap());
        }
        // 更新邮箱
        if update_user.email.is_some() {
            current_user.email = Set(Some(update_user.email.unwrap()));
        }
        // 更新电话
        if update_user.phone.is_some() {
            current_user.phone = Set(update_user.phone.unwrap());
        }
        if update_user.avatar.is_some() {
            current_user.avatar = Set(Some(update_user.avatar.unwrap()));
        }
        // 更新备注
        if update_user.remark.is_some() {
            current_user.remark = Set(Some(update_user.remark.unwrap()));
        }
        // 更新描述
        if update_user.description.is_some() {
            current_user.description = Set(Some(update_user.description.unwrap()));
        }

        // 更新角色
        if update_user.role_id.is_some() {
            current_user.role_id = Set(update_user.role_id.unwrap());
        }

        // 更新时间
        current_user.updated_at = Set(Some(Local::now().naive_local()));
        // 更新数据
        let update_data = current_user.update(&db).await?;
        Ok(update_data.id)
    } else {
        bail!("用户数据不存在，无法更新".to_string())
    }
}

/// 更新用户状态
/// @param id 用户编号
/// @param status 用户状态，值为【0或1】，其他值无效
pub async fn update_status(id: String, status: i32) -> Result<String> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("用户编号不能为空".to_string())
    }
    // 判断是否为超级管理员
    if ADMIN_ID == id {
        bail!("超级用户无法变更状态".to_string())
    }
    // 查询用户信息
    let result = SysUser::find_by_id(id).one(&db).await?;
    // 更新用户名
    if let Some(current_user) = result {
        let mut current_user: ActiveModel = current_user.into();
        match status {
            0 | 1 => current_user.status = Set(status),
            _ => {
                bail!("用户状态值不合法，只能是【0或1】".to_string())
            }
        };
        // 更新时间
        current_user.updated_at = Set(Some(Local::now().naive_local()));
        // 更新状态
        let ok = current_user.update(&db).await?;
        Ok(ok.id)
    } else {
        bail!("用户数据不存在，无法更新".to_string())
    }
}

/// 查询用户列表
/// @param data 类型SearchUserDto
pub async fn search(user: ResponseUser, data: SearchUserDto) -> Result<PageResult<ResponseUser>> {
    let db = get_db().await;
    let mut select = SysUser::find();
    // 判断用户名是否为空
    if data.username.is_some() {
        select = select.filter(SysUserColumn::Username.contains(data.username.unwrap()));
    }
    // 判断昵称是否为空
    if data.nickname.is_some() {
        select = select.filter(SysUserColumn::Nickname.contains(data.nickname.unwrap()));
    }
    // 判断状态是否为空
    if data.status.is_some() {
        select = select.filter(SysUserColumn::Status.eq(data.status.unwrap()));
    }
    // 判断备注是否为空
    if data.remark.is_some() {
        select = select.filter(SysUserColumn::Remark.contains(data.remark.unwrap()));
    }
    // 判断描述是否为空
    if data.description.is_some() {
        select = select.filter(SysUserColumn::Description.contains(data.description.unwrap()));
    }
    // 判断租户是否为空
    if data.tenant_id.is_some() {
        let tenant_id = user.tenant;
        match tenant_id {
            Some(tenant) => select = select.filter(SysUserColumn::TenantId.eq(tenant.id)),
            None => select = select.filter(SysUserColumn::TenantId.eq(data.tenant_id.unwrap())),
        }
    } else if let Some(tenant) = user.tenant {
        select = select.filter(SysUserColumn::TenantId.eq(tenant.id))
    }
    // 排除已删除用户
    select = select.filter(SysUserColumn::DeletedAt.is_null());
    // 排序 todo
    select = select.order_by_desc(SysUserColumn::CreatedAt);
    // 获取事务对象
    // 查询数据数量
    let total = select.clone().count(&db).await?;
    let (page_no, page_size) = handler_page(data.page);

    // 分页总数
    let paginate = select.paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;
    // 查询用户数据
    let user_list = paginate.fetch_page(page_no - 1).await?;

    let mut tenant_ids = Vec::new();
    let mut role_ids = Vec::new();
    for user in user_list.iter() {
        let tenant_id = user.tenant_id.clone();
        if let Some(data) = tenant_id {
            tenant_ids.push(data);
        }
        let role_id = user.role_id.clone();
        role_ids.push(role_id)
    }

    // 查询租户信息
    let tenant_map: HashMap<String, ResponseTenant> = SysTenant::find()
        .filter(SysTenantColumn::Id.is_in(tenant_ids))
        .all(&db)
        .await?
        .iter()
        .map(|item| (item.id.clone(), item.clone().into()))
        .collect::<HashMap<_, _>>();
    // 查询角色信息
    let role_map: HashMap<String, ResponseRole> = SysRole::find()
        .filter(SysRoleColumn::Id.is_in(role_ids))
        .all(&db)
        .await?
        .iter()
        .map(|item| (item.id.clone(), item.clone().into()))
        .collect::<HashMap<_, _>>();

    let mut result = Vec::new();
    // 遍历数据
    for user in user_list.into_iter() {
        let mut data: ResponseUser = user.clone().into();
        // 租户
        if user.tenant_id.is_some() {
            let tenant = tenant_map.get(&user.tenant_id.clone().unwrap());
            if tenant.is_some() {
                data.tenant = Some(tenant.unwrap().clone());
            }
        }
        let role = role_map.get(&user.role_id.clone());
        if role.is_some() {
            data.role = Some(role.unwrap().clone());
        }
        result.push(data);
    }

    let result = PageResult::new(result, total, pages, page_no);
    Ok(result)
}
