use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, TransactionTrait};
use tracing::info;
use uuid::Uuid;
use common::db::get_db;
use common::error::CtsError;
use entity::sys_role::{ActiveModel, Column as SysRoleColumn, Entity as SysRole};
use models::dto::{PageResult, handler_page};
use models::dto::sys::request::sys_role::{AddRoleDto, SearchRoleDto, UpdateRoleDto};
use models::dto::sys::response::sys_role::ResponseRole;
use crate::service::has_tenant;
use crate::service::sys::ADMIN_ID;

/// 根据角色编号查询角色数据
/// @param id 角色编号
pub async fn get_by_id(id: String) -> Result<Option<ResponseRole>, CtsError> {
    let db = get_db().await;
    let result = SysRole::find_by_id(id)
        // 保障删除字段为空
        .filter(SysRoleColumn::DeletedAt.is_null())
        .into_model::<ResponseRole>()
        .one(&db).await;
    match result {
        Ok(data) => {
            Ok(data)
        }
        Err(err) => {
            CtsError::Sql(err.to_string()).into()
        }
    }
}

/// 根据角色编号删除角色数据
/// @param id 角色编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String, CtsError> {
    let db = get_db().await;
    // 查询角色信息
    let result = SysRole::find_by_id(id.clone())
        .one(&db).await?;
    if let Some(data) = result {
        // 查看角色是否为超级管理员
        if ADMIN_ID == data.id {
            info!("超级管理员角色无法删除");
            return CtsError::Custom("超级管理员角色无法删除".to_string()).into();
        } else {
            // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
            match force {
                true => {
                    // 删除角色
                    let delete_result = SysRole::delete_by_id(id)
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
        }
    } else {
        info!("该角色不存在");
        CtsError::DataNotExists("该角色不存在".to_string()).into()
    }
}

/// 添加角色
/// @param 角色对象
pub async fn add(data: AddRoleDto) -> Result<String, CtsError> {
    let db = get_db().await;
    // 判断角色名是否为空
    if data.name.is_empty() {
        return CtsError::Request("名称不能为空".to_string()).into();
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
pub async fn update(id: String, update_role: UpdateRoleDto) -> Result<String, CtsError> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        info!("角色编号不能为空");
        return Err(CtsError::DataNotExists("角色编号不能为空".to_string()));
    }
    if ADMIN_ID == id {
        info!("超级管理员角色无法更新");
        return Err(CtsError::Custom("超级管理员角色无法更新".to_string()));
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
        info!("角色数据不存在，无法更新");
        Err(CtsError::Custom("角色数据不存在，无法更新".to_string()))
    }
}

/// 查询角色列表
/// @param data 类型SearchRoleDto
pub async fn search(data: SearchRoleDto) -> Result<PageResult<ResponseRole>, CtsError> {
    let db = get_db().await;
    let mut select = SysRole::find();
    // 判断名称是否为空
    if data.name.is_some() {
        select = select.filter(SysRoleColumn::Name.contains(data.name.unwrap()))
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
        select = select.filter(SysRoleColumn::TenantId.eq(data.tenant_id.unwrap()))
    }
    // 排除已删除角色
    select = select.filter(SysRoleColumn::DeletedAt.is_null());
    // 获取事务对象
    let tx = db.begin().await?;
    // 查询数据数量
    let total = select.clone().count(&tx).await?;
    let (page_no, page_size) = handler_page(data.page);
    // 分页对象
    let paginate = select
        .into_model::<ResponseRole>()
        .paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;

    // 查询角色数据
    let list = paginate.fetch_page(page_no - 1).await?;
    let result = PageResult::new(list, total, pages, page_no);
    Ok(result)
}