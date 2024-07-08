use anyhow::{bail, Result};
use chrono::Local;
use sea_orm::{ConnectionTrait, DatabaseBackend, FromQueryResult, JsonValue, PaginatorTrait, Statement};
use serde_json::Value;

use common::db::db_type::DbType;
use common::db::form::get_table_name;
use common::db::get_db;
use common::db::select::CtsSelect;
use models::dto::{handler_page, PageResult};
use models::dto::cts::request::task::SearchTask;
use project_form::form::form_util::{parse_value_to_insert_sql, parse_value_to_update_sql};

/// 根据任务编号查询数据
/// @param id 编号
pub async fn get_by_id(table_id: String, id: String) -> Result<Option<Value>> {
    let db = get_db().await;
    let table_name = get_table_name(&table_id, false);
    let result = CtsSelect::table(&table_name).find_by_id(&id, false)
        .one(&db).await?;
    Ok(result)
}

/// 根据任务编号删除数据
/// @param id 编号
pub async fn delete_by_id(table_id: String, id: String, force: bool) -> Result<String> {
    let db = get_db().await;
    // 查询数据是否存在
    let result = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        format!("select * from task_{} where id=$1", &table_id),
        [
            id.clone().into()
        ],
    ))
        .one(&db).await?;
    if result.is_some() {
        // 判断是否强制删除，如果是删除数据，如果不是更新删除字段
        match force {
            true => {
                let sql = format!("DELETE FROM task_{} WHERE id= '{}'", &table_id, &id);
                db.execute_unprepared(&sql).await?;
                Ok(id)
            }
            false => {
                let date = Local::now().naive_local();
                let sql = format!("UPDATE task_{} SET DELETED_AT={} WHERE id= '{}'", &table_id, date.display(), &id);
                db.execute_unprepared(&sql).await?;
                Ok(id)
            }
        }
    } else {
        bail!("无法删除，数据不存在")
    }
}

/// 添加任务
/// @param data任务对象
pub async fn add(task_id: String, data: Value) -> Result<String> {
    // 获取数据库连接
    let db = get_db().await;
    let (uuid, sql) = parse_value_to_insert_sql(task_id, data, false);
    db.execute_unprepared(&sql).await?;
    Ok(uuid)
}

/// 更新任务信息
/// @param data 任务对象
pub async fn update(table_id: String, id: String, data: Value) -> Result<String> {
    // 获取数据库连接
    let db = get_db().await;
    let sql = parse_value_to_update_sql(table_id, id, data, false);
    let result = db.execute_unprepared(&sql).await?;
    match result.rows_affected() {
        1 => {
            Ok("更新成功".to_string())
        }
        _ => {
            Ok("更新失败".to_string())
        }
    }
}

/// 查询项目列表
/// @param data 类型SearchProjectDto
pub async fn search(table_id: String, data: SearchTask) -> Result<PageResult<Value>> {
    let db = get_db().await;
    let table_name = get_table_name(&table_id, false);
    let select = CtsSelect::table(&table_name).find();
    // 查询数据数量
    let total = select.count(&db).await?;
    let (page_no, page_size) = handler_page(data.page);
    // 创建查询对象
    let select = CtsSelect::table(&table_name).find();
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
        .collect();
    let result = PageResult::new(list, total, pages, page_no);
    Ok(result)
}

