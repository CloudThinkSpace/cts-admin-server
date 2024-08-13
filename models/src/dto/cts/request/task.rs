use crate::dto::Page;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTask {
    pub fields: Option<Vec<String>>,
    pub wheres: Option<Vec<String>>,
    // 分页信息
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<String>>,
}
