use anyhow::{bail, Result};
use sea_orm::PaginatorTrait;
use serde_json::Value;

use common::db::form::get_table_name;
use common::db::get_db;
use common::db::select::CtsSelect;
use models::dto::cts::request::task::SearchTask;
use models::dto::{handler_page, PageResult};

const IS_DATA: bool = false;

/// 根据任务编号查询数据
/// @param id 编号
pub async fn get_by_id(table_id: String, id: String) -> Result<Option<Value>> {
    let db = get_db().await;
    let table_name = get_table_name(&table_id, IS_DATA);
    let result = CtsSelect::table(&table_name)
        .find_by_id(&id, false)
        .one(&db)
        .await?;
    Ok(result)
}

/// 根据任务编号删除数据
/// @param id 编号
pub async fn delete_by_id(table_id: String, id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询数据是否存在
    let table_name = get_table_name(&table_id, IS_DATA);
    let result = CtsSelect::table(&table_name)
        .find_by_id(&id, true)
        .one(&db)
        .await?;
    if result.is_some() {
        // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
        let _ = CtsSelect::table(&table_name)
            .delete_by_id(&id, force)
            .execute(&db)
            .await?;
        Ok(id)
    } else {
        bail!("无法删除，数据不存在")
    }
}

/// 添加任务
/// @param data任务对象
pub async fn add(table_id: String, data: Value) -> Result<String> {
    // 获取数据库连接
    let db = get_db().await;
    let table_name = get_table_name(&table_id, IS_DATA);
    let mut data_id = String::from("0");
    let _ = CtsSelect::table(&table_name)
        .add(data, |id| data_id.clone_from(id))?
        .execute(&db)
        .await?;

    Ok(data_id)
}

/// 更新任务信息
/// @param data 任务对象
pub async fn update(table_id: String, id: String, data: Value) -> Result<String> {
    // 获取数据库连接
    let db = get_db().await;
    // 查询是否有数据
    let table_name = get_table_name(&table_id, IS_DATA);
    let result = CtsSelect::table(&table_name)
        .find_by_id(&id, false)
        .one(&db)
        .await?;
    match result {
        None => {
            bail!("数据不存在，无法更新")
        }
        Some(_) => {
            let _ = CtsSelect::table(&table_name)
                .update(&id, data)?
                .execute(&db)
                .await?;
            Ok(id)
        }
    }
}

/// 查询项目列表
/// @param data 类型 SearchTask
pub async fn search(table_id: String, data: SearchTask) -> Result<PageResult<Value>> {
    let db = get_db().await;
    let table_name = get_table_name(&table_id, IS_DATA);
    let select = CtsSelect::table(&table_name).select();
    // 查询数据数量
    let total = select.count(&db).await?;
    let (page_no, page_size) = handler_page(data.page);
    // 创建查询对象
    let select = CtsSelect::table(&table_name).select();
    // 分页对象
    let paginate = select.paginate(&db, page_size);
    // 页数
    let pages = paginate.num_pages().await?;

    // 查询Api数据
    let list = paginate
        .fetch_page(page_no - 1)
        .await?
        .into_iter()
        .collect();
    let result = PageResult::new(list, total, pages, page_no);
    Ok(result)
}
