use serde::Deserialize;
use crate::dto::{Order, Page};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFormData {
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<Order>>,
}