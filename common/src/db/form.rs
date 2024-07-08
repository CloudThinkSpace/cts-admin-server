

/// 获取完整表名
/// @param table_id 表id
/// @param is_data 是否数据表
pub fn get_table_name(table_id:&str, is_data:bool) -> String {
    match is_data {
        true => {
            format!("data_{}", table_id)
        }
        false => {
            format!("task_{}", table_id)
        }
    }
}
