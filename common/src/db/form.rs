use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use anyhow::{bail, Result};
use chrono::Local;
use serde_json::Value;
use tracing::info;
use uuid::Uuid;

use crate::db::{insert_data_sql, update_data_sql};
use crate::db::db_type::{DbType, Null};

/// 获取完整表名
/// @param table_id 表id
/// @param is_data 是否数据表
pub fn get_table_name(table_id: &str, is_data: bool) -> String {
    match is_data {
        true => {
            format!("data_{}", table_id)
        }
        false => {
            format!("task_{}", table_id)
        }
    }
}


pub enum FormCommonField {
    Id,
    Code,
    Lon,
    Lat,
    Status,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

impl Display for FormCommonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormCommonField::Id => {
                write!(f, "id")
            }
            FormCommonField::Code => {
                write!(f, "code")
            }
            FormCommonField::Lon => {
                write!(f, "lon")
            }
            FormCommonField::Lat => {
                write!(f, "lat")
            }
            FormCommonField::Status => {
                write!(f, "status")
            }
            FormCommonField::UserId => {
                write!(f, "user_id")
            }
            FormCommonField::CreatedAt => {
                write!(f, "created_at")
            }
            FormCommonField::UpdatedAt => {
                write!(f, "updated_at")
            }
            FormCommonField::DeletedAt => {
                write!(f, "deleted_at")
            }
        }
    }
}

impl FormCommonField {
    pub fn contains(data: &str) -> bool {
        match data {
            "id" | "code" | "lon" | "lat" | "status" | "user_id" | "created_at" | "updated_at" | "deleted_at" => {
                true
            }
            &_ => {
                false
            }
        }
    }

    pub fn vec() -> Vec<FormCommonField>{
        vec![
            FormCommonField::Id,
            FormCommonField::Code,
            FormCommonField::Lon,
            FormCommonField::Lat,
            FormCommonField::Status,
            FormCommonField::UserId,
            FormCommonField::CreatedAt,
            FormCommonField::UpdatedAt,
            FormCommonField::DeletedAt,
        ]
    }
}


/// 解析表单数据对象
pub fn parse_data(data: Value) -> Result<HashMap<String, Box<dyn DbType>>> {
    let mut result = HashMap::new();
    match data {
        Value::Object(item) => {
            for (key, value) in item.into_iter() {
                let value_opt = parse_value(value);
                if let Some(opt) = value_opt {
                    result.insert(key, opt);
                }
            }
        }
        _ => {
            bail!("data 数据必须是HashMap格式")
        }
    }
    Ok(result)
}

fn parse_value(data: Value) -> Option<Box<dyn DbType>> {
    match data {
        Value::Null => {
            Some(Box::new(Null))
        }
        Value::Bool(item) => {
            Some(Box::new(item))
        }
        Value::Number(item) => {
            if item.is_f64() {
                Some(Box::new(item.as_f64().unwrap()))
            } else {
                Some(Box::new(item.as_i64().unwrap()))
            }
        }
        Value::String(item) => {
            Some(Box::new(item))
        }
        _ => {
            None
        }
    }
}

/// 解析value数据生成插入sql
/// @param table_name 表名
/// @param data 提交的数据
/// @param is_data 是否是表单数据，如果不是，代表任务数据
pub fn parse_value_to_insert_sql<F, P>(
    table_name: String,
    data: Value,
    validate: F,
    push: P,
) -> Result<(String, String)>
where
    F: Fn(&HashMap<String, Box<dyn DbType>>) -> Result<()>,
    P: Fn(&mut Vec<String>, &mut Vec<Box<dyn DbType>>),
{
    // 解析任务数据
    let map_data = parse_data(data)?;
    // todo 检查字段是否有效
    validate(&map_data)?;
    let mut headers = Vec::new();
    let mut columns = Vec::new();
    let mut uuid= "".to_string();
    // 分解字段列表和数据列表
    for (key, item) in map_data.into_iter() {
        // 判断是否为 id
        if key == FormCommonField::Id.to_string() {
            uuid = item.source()
        }
        headers.push(key);
        columns.push(item);

    }

    // 插入主键、状态、创建时间
    headers.push(FormCommonField::CreatedAt.to_string());
    // 判断主键是否存在，不存在插入uuid
    if !headers.contains(&FormCommonField::Id.to_string()) {
        headers.insert(0, FormCommonField::Id.to_string());
        uuid = Uuid::new_v4().to_string();
        columns.insert(0, Box::new(uuid.clone()));
    }
    let date = Local::now().naive_local();
    columns.push(Box::new(date));
    // 判断是否添加其他字段，
    push(&mut headers, &mut columns);
    // 检查两个列表长度是否一致
    if headers.len() != columns.len() {
        bail!("表头列表和数据列表添加数量必须一致")
    }
    // 生成插入数据sql
    let insert_sql = insert_data_sql(&table_name, &headers, &columns);
    info!("{}", insert_sql);
    Ok((uuid, insert_sql))
}

/// 解析value数据生成更新sql
/// @param table_name 表名
/// @param id 更新数据的编号
/// @param data 提交的数据
/// @param validate 验证数据是否满足要求
pub fn parse_value_to_update_sql<F>(
    table_name: String,
    id: String,
    data: Value,
    validate: F,
) -> Result<String>
where
    F: Fn(&HashMap<String, Box<dyn DbType>>) -> Result<()>,
{
    // 解析任务数据
    let mut map_data = parse_data(data)?;
    // todo 检查字段是否有效
    validate(&map_data)?;
    // 任务表名
    // 插入时间值
    let date = Local::now().naive_local();
    map_data.insert(FormCommonField::UpdatedAt.to_string(), Box::new(date));
    // 生成插入数据sql
    let update_sql = update_data_sql(&table_name, &id, map_data);
    Ok(update_sql)
}