use anyhow::{bail, Result};
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use common::db::get_db;
use entity::form_template::{ActiveModel, Column as FormTemplateColumn, Entity as FormTemplate};
use models::dto::{handler_page, PageResult};
use models::dto::cts::request::form_template::{AddFormTemplateDto, SearchFormTemplateDto, UpdateFormTemplateDto};
use models::dto::cts::response::form_template::ResponseFormTemplate;

/// 根据表单编号查询数据
/// @param id 编号
pub async fn get_by_id(id: String) -> Result<Option<ResponseFormTemplate>> {
    let db = get_db().await;
    let result = FormTemplate::find_by_id(id.clone())
        // 保障删除字段为空
        .filter(FormTemplateColumn::DeletedAt.is_null())
        .one(&db).await?;
    match result {
        None => {
            bail!("编号：{}，数据不存在",id)
        }
        Some(data) => {
            let role: ResponseFormTemplate = data.into();
            // formTemplate数据
            Ok(Some(role))
        }
    }
}

/// 根据表单编号删除数据
/// @param id 编号
pub async fn delete_by_id(id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询表单信息
    let result = FormTemplate::find_by_id(id.clone()).one(&db).await?;
    if let Some(data) = result {
        // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
        match force {
            true => {
                // 删除表单
                let delete_result = FormTemplate::delete_by_id(id)
                    .exec(&db)
                    .await?;
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
        bail!("该表单不存在".to_string())
    }
}

/// 添加表单
/// @param 表单对象
pub async fn add(data: AddFormTemplateDto) -> Result<String> {
    let db = get_db().await;
    // 判断Api名是否为空
    if data.name.is_empty() {
        bail!("名称不能为空".to_string())
    }

    let current: ActiveModel = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(data.name),
        title: Set(data.title),
        content: Set(data.content),
        remark: Set(data.remark),
        version: Set(data.version),
        description: Set(data.description),
        created_at: Set(Local::now().naive_local()),
        updated_at: NotSet,
        deleted_at: NotSet,
    };

    let add_role = current.insert(&db).await?;
    Ok(add_role.id)
}

/// 更新表单信息
/// @param update_form_template 待更新的表单对象
pub async fn update(id: String, update_form_template: UpdateFormTemplateDto) -> Result<String> {
    let db = get_db().await;
    // 判断id是否存在
    if id.is_empty() {
        bail!("编号不能为空".to_string())
    }
    // 查询表单信息
    let result = FormTemplate::find_by_id(id).one(&db).await?;
    // 更新表单信息
    if let Some(current) = result {
        let mut current: ActiveModel = current.into();
        // 名称是否为空
        if update_form_template.name.is_some() {
            current.name = Set(update_form_template.name.unwrap());
        }
        // 更新title
        if update_form_template.title.is_some() {
            current.title = Set(update_form_template.title.unwrap())
        }
        // 更新content
        if update_form_template.content.is_some() {
            current.content = Set(update_form_template.content.unwrap())
        }
        // 更新 version
        if update_form_template.version.is_some() {
            current.version = Set(update_form_template.version.unwrap())
        }
        // 更新备注
        if update_form_template.remark.is_some() {
            current.remark = Set(Some(update_form_template.remark.unwrap()))
        }
        // 更新描述
        if update_form_template.description.is_some() {
            current.description = Set(Some(update_form_template.description.unwrap()))
        }
        // 更新时间
        current.updated_at = Set(Some(Local::now().naive_local()));
        // 更新数据
        let update_data = current.update(&db).await?;
        Ok(update_data.id)
    } else {
        bail!("表单数据不存在，无法更新".to_string())
    }
}

/// 查询表单列表
/// @param data 类型SearchFormTemplateDto
pub async fn search(data: SearchFormTemplateDto) -> Result<PageResult<ResponseFormTemplate>> {
    let db = get_db().await;
    let mut select = FormTemplate::find();
    // 判断名称是否为空
    if data.name.is_some() {
        select = select.filter(FormTemplateColumn::Name.contains(data.name.unwrap()))
    }
    // 判断备注是否为空
    if data.remark.is_some() {
        select = select.filter(FormTemplateColumn::Remark.contains(data.remark.unwrap()));
    }
    // 判断描述是否为空
    if data.description.is_some() {
        select = select.filter(FormTemplateColumn::Description.contains(data.description.unwrap()));
    }
    // 判断title是否为空
    if data.title.is_some() {
        select = select.filter(FormTemplateColumn::Title.eq(data.title.unwrap()))
    }
    // 判断content是否为空
    if data.content.is_some() {
        select = select.filter(FormTemplateColumn::Content.eq(data.content.unwrap()))
    }
    // 判断version是否为空
    if data.version.is_some() {
        select = select.filter(FormTemplateColumn::Version.eq(data.version.unwrap()))
    }
    // 排除已删除角色
    select = select.filter(FormTemplateColumn::DeletedAt.is_null());
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