use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMenu {
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub sort: Option<i64>,
    pub path: Option<String>,
    pub hidden: i32,
    pub component: Option<String>,
    pub active_name: Option<String>,
    pub keep_alive: i32,
    pub title: String,
    pub icon: Option<String>,
    pub default_menu: i32,
    pub menu_level: i64,
    pub close_tab: i32,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub children: Vec<ResponseMenu>,
}