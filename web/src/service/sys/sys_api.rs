use anyhow::{bail, Result};
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use common::db::get_db;
use entity::sys_api::{ActiveModel, Column as SysApiColumn, Entity as SysApi};
use models::dto::{handler_page, PageResult};
use models::dto::sys::request::sys_api::{AddApiDto, SearchApiDto, UpdateApiDto};
use models::dto::sys::response::sys_api::ResponseApi;

/// 根据Api编号查询数据
/// @param id 编号
pub async fn get_by_id(id: String) -> Result<Option<ResponseApi>> {
    let db = get_db().await;
    let result = SysApi::find_by_id(id.clone())
        // 保障删除字段为空
        .filter(SysApiColumn::DeletedAt.is_null())
        .one(&db).await?;
    match result {
        None => {
            bail!("编号：{}，数据不存在",id)
        }
        Some(data) => {
            let role: ResponseApi = data.into();
            // api数据
            Ok(Some(role))
        }
    }
}

/// 根据Api编号删除数据
/// @param id 编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询角色信息
    let result = SysApi::find_by_id(id.clone()).one(&db).await?;
    if let Some(data) = result {
        // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
        match force {
            true => {
                // 删除Api
                let delete_result = SysApi::delete_by_id(id)
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
        bail!("该Api不存在".to_string())
    }
}

/// 添加Api
/// @param Api对象
pub async fn add(data: AddApiDto) -> Result<String> {
    let db = get_db().await;
    // 判断Api名是否为空
    if data.name.is_empty() {
        bail!("名称不能为空".to_string())
    }

    let current: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(data.name),
        api_path: Set(data.api_path),
        api_group: Set(data.api_group),
        remark: Set(data.remark),
        description: Set(data.description),
        created_at: Set(Local::now().naive_local()),
        updated_at: NotSet,
        deleted_at: NotSet,
        api_method: Set(data.api_method),
    };

    let add_role = current.insert(&db).await?;
    Ok(add_role.id)
}

/// 更新Api信息
/// @param update_api 待更新的Api对象
pub async fn update(id: String, update_api: UpdateApiDto) -> Result<String> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("编号不能为空".to_string())
    }
    // 查询租户信息
    let result = SysApi::find_by_id(id).one(&db).await?;
    // 更新组织信息
    if let Some(current) = result {
        let mut current: ActiveModel = current.into();
        // 名称是否为空
        if update_api.name.is_some() {
            current.name = Set(update_api.name.unwrap());
        }
        // 更新api path
        if update_api.api_path.is_some() {
            current.api_path = Set(update_api.api_path.unwrap())
        }
        // 更新 api group
        if update_api.api_group.is_some() {
            current.api_group = Set(update_api.api_group.unwrap())
        }
        // 更新 api method
        if update_api.api_method.is_some() {
            current.api_method = Set(update_api.api_method.unwrap())
        }
        // 更新备注
        if update_api.remark.is_some() {
            current.remark = Set(Some(update_api.remark.unwrap()))
        }
        // 更新描述
        if update_api.description.is_some() {
            current.description = Set(Some(update_api.description.unwrap()))
        }
        // 更新时间
        current.updated_at = Set(Some(Local::now().naive_local()));
        // 更新数据
        let update_data = current.update(&db).await?;
        Ok(update_data.id)
    } else {
        bail!("Api数据不存在，无法更新".to_string())
    }
}

/// 查询Api列表
/// @param data 类型SearchApiDto
pub async fn search(data: SearchApiDto) -> Result<PageResult<ResponseApi>> {
    let db = get_db().await;
    let mut select = SysApi::find();
    // 判断名称是否为空
    if data.name.is_some() {
        select = select.filter(SysApiColumn::Name.contains(data.name.unwrap()))
    }
    // 判断备注是否为空
    if data.remark.is_some() {
        select = select.filter(SysApiColumn::Remark.contains(data.remark.unwrap()));
    }
    // 判断描述是否为空
    if data.description.is_some() {
        select = select.filter(SysApiColumn::Description.contains(data.description.unwrap()));
    }
    // 判断api path是否为空
    if data.api_path.is_some() {
        select = select.filter(SysApiColumn::ApiPath.eq(data.api_path.unwrap()))
    }
    // 判断api method是否为空
    if data.api_method.is_some() {
        select = select.filter(SysApiColumn::ApiMethod.eq(data.api_method.unwrap()))
    }
    // 判断api group是否为空
    if data.api_group.is_some() {
        select = select.filter(SysApiColumn::ApiGroup.eq(data.api_group.unwrap()))
    }
    // 排除已删除角色
    select = select.filter(SysApiColumn::DeletedAt.is_null());
    // 查询数据数量
    let total = select.clone().count(&db).await?;
    let (page_no, page_size) = handler_page(data.page);
    // 分页对象
    let paginate = select
        .paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;

    // 查询Api数据
    let list = paginate.
        fetch_page(page_no - 1)
        .await?
        .into_iter()
        .map(|item| item.into())
        .collect();
    let result = PageResult::new(list, total, pages, page_no);
    Ok(result)
}