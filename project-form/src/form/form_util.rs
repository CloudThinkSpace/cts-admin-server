use std::collections::HashMap;
use std::string::ToString;

use anyhow::Result;
use chrono::Local;
use serde_json::Value;
use uuid::Uuid;

use common::db::db_type::{DbType, Null};
use common::db::{insert_data_sql, update_data_sql};

use crate::form::form_json::FormTemplate;
use crate::form::FormCommonField;

const COMMON_FIELDS: [&'static str; 9] = [
    "id",
    "code",
    "lon",
    "lat",
    "status",
    "user_id",
    "created_at",
    "updated_at",
    "deleted_at",
];

pub fn parse(form_str: &str) -> Result<FormTemplate> {
    let form: FormTemplate = serde_json::from_str(form_str)?;
    Ok(form)
}

/// 处理表头函数
/// 判断表头列表是否有公共字段，如果有重命名
/// 添加公共字段
pub fn handler_form_header(headers: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for header in headers.iter() {
        let field = if COMMON_FIELDS.contains(&header.as_str()) {
            handler_common_field(header, 0)
        } else {
            header.to_string()
        };
        result.push(field);
    }
    // 第一个位置插入id字段
    result.insert(0, "id".to_string());
    // 插入其他公共字段，跳过第一个自动
    for field in COMMON_FIELDS.iter().skip(1) {
        result.push(field.to_string());
    }
    result
}

/// 从表头列表中找出对应位置列表
pub fn filter_code_lon_lat(headers: &Vec<String>, fields: &Vec<String>) -> Vec<usize> {
    let mut result = Vec::new();
    for (index, header) in headers.iter().enumerate() {
        if fields.contains(header) {
            result.push(index);
        }
    }
    result
}

/// 处理表单数据
/// 将String类型转换成DbType
/// @param data 数据
/// @param index_common，code，lon，lat 位置数组
pub fn handler_form_data(data: &Vec<String>, index_common: &Vec<usize>) -> Vec<Box<dyn DbType>> {
    let mut result: Vec<Box<dyn DbType>> = Vec::new();
    for datum in data.iter() {
        result.push(Box::new(datum.to_string()));
    }
    // 添加编号
    result.insert(0, Box::new(Uuid::new_v4().to_string()));
    // 从数据中添加code，lon，lat值
    for index in index_common.iter() {
        let location = index.to_owned();
        let value = &result[location];
        result.push(Box::new(value.source()));
    }
    // 添加状态
    result.push(Box::new(0));
    // 添加用户信息
    result.push(Box::new(Null));
    // 添加创建时间、更新时间、删除时间
    result.push(Box::new(Local::now().naive_local()));
    result.push(Box::new(Null));
    result.push(Box::new(Null));

    result
}

/// 处理公共字段，
/// 如果有相同字段，添加_index后缀，【index】自增长
pub fn handler_common_field(name: &str, index: i32) -> String {
    let field_name = format!("{name}_{index}");
    if COMMON_FIELDS.contains(&field_name.as_str()) {
        handler_common_field(name, index + 1)
    } else {
        field_name
    }
}

/// 解析表单数据对象
pub fn parse_data(data: Value) -> HashMap<String, Box<dyn DbType>> {
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
        _ => {}
    }
    result
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
/// @param table_id 表uuid，任务表：task_[table_id],数据表：data_[table_id]
/// @param data 提交的数据
/// @param is_data 是否是表单数据，如果不是，代表任务数据
pub fn parse_value_to_insert_sql(table_id: String, data: Value, is_data:bool) -> (String, String) {
    // 解析任务数据
    let task_data = parse_data(data);
    // todo 检查字段是否有效
    // 任务表名
    let table_name = if is_data { format!("data_{}", &table_id) } else { format!("task_{}", &table_id) };
    let mut headers = Vec::new();
    let mut columns = Vec::new();
    // 分解字段列表和数据列表
    for (key, item) in task_data.into_iter() {
        headers.push(key);
        columns.push(item);
    }
    // 插入主键、状态、创建时间
    headers.insert(0, FormCommonField::Id.to_string());
    headers.push(FormCommonField::Status.to_string());
    headers.push(FormCommonField::CreatedAt.to_string());
    // 插入主键、状态、时间值
    let uuid = Uuid::new_v4().to_string();
    columns.insert(0, Box::new(uuid.clone()));
    let date = Local::now().naive_local();
    columns.push(Box::new(0));
    columns.push(Box::new(date));
    // 生成插入数据sql
    let insert_sql = insert_data_sql(&table_name, &headers, &columns);
    (uuid, insert_sql)
}

pub fn parse_value_to_update_sql(table_id: String, id:String, data: Value, is_data:bool) -> String {
    // 解析任务数据
    let mut  map_data = parse_data(data);
    // todo 检查字段是否有效
    // 任务表名
    let table_name = if is_data { format!("data_{}", &table_id) } else { format!("task_{}", &table_id) };
    // 插入时间值
    let date = Local::now().naive_local();
    map_data.insert(FormCommonField::UpdatedAt.to_string(), Box::new(date));
    // 生成插入数据sql
    let update_sql = update_data_sql(&table_name, &id, map_data);
    update_sql
}