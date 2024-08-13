use anyhow::{bail, Ok, Result};
use sea_orm::{PaginatorTrait, TransactionTrait};
use serde_json::{Map, Value};

use common::db::form::get_table_name;
use common::db::get_db;
use common::db::select::CtsSelect;
use models::dto::cts::request::form::{AddFormData, SearchFormData, UpdateFormData};
use models::dto::{handler_page, PageResult};

const IS_DATA: bool = true;

/// 根据编号查询数据
/// @param id 编号
pub async fn get_by_id(table_id: String, id: String) -> Result<Option<Value>> {
    let db = get_db().await;
    let table_name = get_table_name(&table_id, IS_DATA);
    let mut cts_select = CtsSelect::table(&table_name);
    cts_select.defualt_filter();
    let result = cts_select.find_by_id(&id).one(&db).await?;
    Ok(result)
}

pub async fn get_by_code(table_id: String, code: String) -> Result<Option<Value>> {
    let db = get_db().await;
    let table_name = get_table_name(&table_id, IS_DATA);
    let mut cts_select = CtsSelect::table(&table_name);
    cts_select.defualt_filter();
    cts_select.filter(&format!("code ='{}'", code));
    let result = cts_select.find().one(&db).await?;
    Ok(result)
}

/// 根据编号删除数据
/// @param id 编号
pub async fn delete_by_id(table_id: String, id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询数据是否存在
    let table_name = get_table_name(&table_id, IS_DATA);
    let result = CtsSelect::table(&table_name)
        .find_by_id(&id)
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

/// 添加数据
/// @param data 数据对象
pub async fn add(table_id: String, data: Value) -> Result<String> {
    // 获取数据库连接
    let db = get_db().await;
    let table_name = get_table_name(&table_id, IS_DATA);
    let mut data_id = String::from("0");
    let _ = CtsSelect::table(&table_name)
        .add(data, |id| {
            data_id.clone_from(id);
        })?
        .execute(&db)
        .await?;

    Ok(data_id)
}

// 添加数据2
pub async fn add_data(data: AddFormData) -> Result<String> {
    // 获取数据库连接
    let db = get_db().await;
    let table_id = data.name;
    let table_name = get_table_name(&table_id, true);
    // 添加 id 字段
    let data_value = insert_id_to_data(data.data, &data.task_id);
    // 事务
    let tx = db.begin().await?;
    let _ = CtsSelect::table(&table_name)
        .add(data_value, |data_id|{})?
        .execute(&tx)
        .await?;
    // 更新任务状态
    let mut  map_value = Map::new();
    map_value.insert("status".to_string(),Value::Number(1i32.into()));
    // 任务表
    let task_name = get_table_name(&table_id, false);
    let _ = CtsSelect::table(&task_name)
        .update(&data.task_id, Value::Object(map_value))?
        .execute(&tx)
        .await?;
    tx.commit().await?;
    Ok(data.task_id)

}

pub fn insert_id_to_data(mut data:Value, id:&str) -> Value{
    match data {
        Value::Object(mut value_map) => {
            value_map.insert("id".to_string(),Value::String(id.to_string()));
            Value::Object(value_map)
        }
        _=>{
            data
        }
    }
}

// 更新数据2
pub async fn update_data(data: UpdateFormData) -> Result<String> {
    let table_id = data.name;
    let data_id = data.task_id;
    let form_data = data.data;
    update(table_id, data_id, form_data).await
}

/// 更新数据信息
/// @param data 数据对象
pub async fn update(table_id: String, id: String, data: Value) -> Result<String> {
    // 获取数据库连接
    let db = get_db().await;
    // 查询是否有数据
    let table_name = get_table_name(&table_id, IS_DATA);
    let result = CtsSelect::table(&table_name)
        .find_by_id(&id)
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

/// 查询数据列表
/// @param data 类型SearchFormData
pub async fn search(table_id: String, data: SearchFormData) -> Result<PageResult<Value>> {
    let db = get_db().await;
    let table_name = get_table_name(&table_id, IS_DATA);
    let mut cts_select = CtsSelect::table(&table_name);
    cts_select.columns(data.fields);
    // 设置查询条件
    if let Some(wheres) = data.wheres {
        for where_str in wheres.iter() {
            cts_select.filter(where_str);
        }
    }
    cts_select.defualt_filter();
    // 设置排序
    if let Some(order_bys) = data.orders {
        for order_by in order_bys.iter() {
            cts_select.order_by(order_by);
        }
    }
    let select = cts_select.select();
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
