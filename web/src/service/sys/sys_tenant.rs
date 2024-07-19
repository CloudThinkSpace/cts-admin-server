use anyhow::{bail, Result};
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use sea_orm::QueryOrder;
use common::db::get_db;
use entity::sys_tenant::{ActiveModel, Column as SysTenantColumn, Entity as SysTenant};
use models::dto::{handler_page, PageResult};
use models::dto::sys::request::sys_tenant::{AddTenantDto, SearchTenantDto, UpdateTenantDto};
use models::dto::sys::response::sys_tenant::ResponseTenant;

/// 根据租户编号查询数据
/// @param id 租户编号
pub async fn get_by_id(id: String) -> Result<Option<ResponseTenant>> {
    let db = get_db().await;
    let result = SysTenant::find_by_id(id.clone())
        // 保障删除字段为空
        .filter(SysTenantColumn::DeletedAt.is_null())
        .one(&db).await?;
    match result {
        None => {
            bail!("编号：{}，数据不存在",id)
        }
        Some(data) => {
            let role: ResponseTenant = data.into();
            // 租户数据
            Ok(Some(role))
        }
    }
}

/// 根据租户编号删除数据
/// @param id 租户编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询角色信息
    let result = SysTenant::find_by_id(id.clone()).one(&db).await?;
    if let Some(data) = result {
        // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
        match force {
            true => {
                // 删除角色
                let delete_result = SysTenant::delete_by_id(id)
                    .exec(&db).await?;
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
    } else {
        bail!("该租户不存在".to_string())
    }
}

/// 添加租户
/// @param 租户对象
pub async fn add(data: AddTenantDto) -> Result<String> {
    let db = get_db().await;
    // 判断角色名是否为空
    if data.name.is_empty() {
        bail!("名称不能为空".to_string())
    }

    let current: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(data.name),
        enabled: Set(0),
        remark: Set(data.remark),
        description: Set(data.description),
        created_at: Set(Local::now().naive_local()),
        updated_at: NotSet,
        deleted_at: NotSet,
        r#type: Set(data.r#type),
    };

    let add_role = current.insert(&db).await?;
    Ok(add_role.id)
}

/// 更新租户信息
/// @param update_role 待更新的租户对象
pub async fn update(id: String, update_tenant: UpdateTenantDto) -> Result<String> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("编号不能为空".to_string())
    }
    // 查询租户信息
    let result = SysTenant::find_by_id(id).one(&db).await?;
    // 更新组织信息
    if let Some(current) = result {
        let mut current: ActiveModel = current.into();
        // 名称是否为空
        if update_tenant.name.is_some() {
            current.name = Set(update_tenant.name.unwrap());
        }
        // 更新enabled
        if update_tenant.enabled.is_some() {
            current.enabled = Set(update_tenant.enabled.unwrap())
        }
        // 更新type
        if update_tenant.r#type.is_some() {
            current.r#type = Set(update_tenant.r#type.unwrap())
        }
        // 更新备注
        if update_tenant.remark.is_some() {
            current.remark = Set(Some(update_tenant.remark.unwrap()))
        }
        // 更新描述
        if update_tenant.description.is_some() {
            current.description = Set(Some(update_tenant.description.unwrap()))
        }
        // 更新时间
        current.updated_at = Set(Some(Local::now().naive_local()));
        // 更新数据
        let update_data = current.update(&db).await?;
        Ok(update_data.id)
    } else {
        bail!("租户数据不存在，无法更新".to_string())
    }
}

/// 更新状态
/// @param id tenant编号
/// @param status 状态，值为【0或1】，其他值无效
pub async fn update_status(id: String, status: i32) -> Result<String> {
    let db: sea_orm::DatabaseConnection = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("租户编号不能为空".to_string())
    }

    // 查询租户信息
    let result = SysTenant::find_by_id(id).one(&db).await?;
    // 更新用户名
    if let Some(current_data) = result {
        let mut current_data: ActiveModel = current_data.into();
        match status {
            0 | 1 => current_data.enabled = Set(status),
            _ => {
                bail!("租户状态值不合法，只能是【0或1】".to_string())
            }
        };
        // 更新时间
        current_data.updated_at = Set(Some(Local::now().naive_local()));
        // 更新状态
        let ok = current_data.update(&db).await?;
        Ok(ok.id)
    } else {
        bail!("租户数据不存在，无法更新".to_string())
    }
}

/// 查询租户列表
/// @param data 类型SearchTenantDto
pub async fn search(data: SearchTenantDto) -> Result<PageResult<ResponseTenant>> {
    let db = get_db().await;
    let mut select = SysTenant::find();
    // 判断名称是否为空
    if data.name.is_some() {
        select = select.filter(SysTenantColumn::Name.contains(data.name.unwrap()))
    }
    // 判断备注是否为空
    if data.remark.is_some() {
        select = select.filter(SysTenantColumn::Remark.contains(data.remark.unwrap()));
    }
    // 判断描述是否为空
    if data.description.is_some() {
        select = select.filter(SysTenantColumn::Description.contains(data.description.unwrap()));
    }
    // 判断enabled是否为空
    if data.enabled.is_some() {
        select = select.filter(SysTenantColumn::Enabled.eq(data.enabled.unwrap()))
    }
    // 判断type是否为空
    if data.r#type.is_some() {
        select = select.filter(SysTenantColumn::Type.eq(data.r#type.unwrap()))
    }
    // 排除已删除角色
    select = select.filter(SysTenantColumn::DeletedAt.is_null());
    // 排序 todo
    select = select.order_by_desc(SysTenantColumn::CreatedAt);
    // 查询数据数量
    let total = select.clone().count(&db).await?;
    let (page_no, page_size) = handler_page(data.page);
    // 分页对象
    let paginate = select
        .paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;

    // 查询角色数据
    let list = paginate.
        fetch_page(page_no - 1)
        .await?
        .into_iter()
        .map(|item| item.into())
        .collect();
    let result = PageResult::new(list, total, pages, page_no);
    Ok(result)
}
