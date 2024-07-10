use chrono::Local;
use uuid::Uuid;
use common::db::db_type::{DbType, Null};
use common::db::form::FormCommonField;
use crate::form::form_json::FormTemplate;

pub fn parse(form_str: &str) -> anyhow::Result<FormTemplate> {
    let form: FormTemplate = serde_json::from_str(form_str)?;
    Ok(form)
}


/// 处理表头函数
/// 判断表头列表是否有公共字段，如果有重命名
/// 添加公共字段
pub fn handler_form_header(headers: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for header in headers.iter() {
        let field = if FormCommonField::contains(header.as_str()) {
            handler_common_field(header, 0)
        } else {
            header.to_string()
        };
        result.push(field);
    }
    // 第一个位置插入id字段
    result.insert(0, "id".to_string());
    // 插入其他公共字段，跳过第一个自动
    for field in FormCommonField::vec().iter().skip(1) {
        result.push(field.to_string());
    }
    result
}

/// 从表头列表中找出对应位置列表
pub fn filter_code_lon_lat(headers: &[String], fields: &[String]) -> Vec<usize> {
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
pub fn handler_form_data(data: &[String], index_common: &[usize]) -> Vec<Box<dyn DbType>> {
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
    if FormCommonField::contains(field_name.as_str()) {
        handler_common_field(name, index + 1)
    } else {
        field_name
    }
}
