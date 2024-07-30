use crate::dto::{Order, Page};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTask {
    // 编号
    pub code: Option<String>,
    // 状态
    pub status: Option<i32>,
    // 分页信息
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<Order>>,
}

