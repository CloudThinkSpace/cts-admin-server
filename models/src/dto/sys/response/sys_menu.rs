use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use entity::sys_menu::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMenu {
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub sort: i64,
    pub path: String,
    pub hidden: i32,
    pub component: String,
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
    pub children: Option<Vec<ResponseMenu>>,
}

impl From<Model> for ResponseMenu {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            path: value.path,
            parent_id: value.parent_id,
            hidden: value.hidden,
            component: value.component,
            active_name: value.active_name,
            keep_alive: value.keep_alive,
            title: value.title,
            icon: value.icon,
            default_menu: value.default_menu,
            menu_level: value.menu_level,
            close_tab: value.close_tab,
            description: value.description,
            remark: value.remark,
            created_at: value.created_at,
            updated_at: value.updated_at,
            children: None,
            sort: value.sort,
        }
    }
}