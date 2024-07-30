use anyhow::{Ok, Result};
use common::db::{form::get_table_name, get_db, select::CtsSelect};
use serde_json::Value;

pub async fn get_table_fields(table_id: String, param: Option<bool>) -> Result<Vec<Value>> {
    let db = get_db().await;
    // 获取表名
    let table_name = match param {
        Some(data) => get_table_name(&table_id, data),
        None => table_id,
    };

    let result = CtsSelect::table(&table_name)
        .find_table_field()
        .all(&db)
        .await?;
    Ok(result)
}
