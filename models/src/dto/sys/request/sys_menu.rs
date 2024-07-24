use crate::dto::{Order, Page};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMenuDto {
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub sort: Option<i64>,
    pub path: Option<String>,
    pub hidden: Option<bool>,
    pub component: Option<String>,
    pub active_name: Option<String>,
    pub keep_alive: Option<bool>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub default_menu: Option<bool>,
    pub menu_level: Option<i64>,
    pub close_tab: Option<bool>,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct AddMenuDto {
    pub name: String,
    pub parent_id: String,
    pub sort: i64,
    pub path: String,
    pub hidden: bool,
    pub component: String,
    pub active_name: Option<String>,
    pub keep_alive: bool,
    pub title: String,
    pub icon: Option<String>,
    pub close_tab: bool,
    pub description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMenuDto {
    pub name: Option<String>,
    pub path: Option<String>,
    pub hidden: Option<bool>,
    pub component: Option<String>,
    pub active_name: Option<String>,
    pub keep_alive: Option<bool>,
    pub title: Option<String>,
    pub default_menu: Option<bool>,
    pub menu_level: Option<i64>,
    pub close_tab: Option<bool>,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub page: Option<Page>,
    // 排序
    pub orders: Option<Vec<Order>>,
}
