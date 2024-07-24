use common::date_time_format;
use common::date_time_format_option;
use entity::sys_menu::Model;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

const SYSTEM_PARENT_MENU_ID: &str = "";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub title: String,
    pub icon: Option<String>,
    pub single: Option<String>,
    pub img: Option<String>,
    pub affix: Option<String>,
    pub roles: Option<String>,
    pub is_link: Option<String>,
    pub hide_tab: Option<bool>,
    pub order_no: Option<i64>,
    pub frame_src: Option<String>,
}

impl Meta {
    pub fn new(title: String, icon: Option<String>, order_no: Option<i64>) -> Self {
        Self {
            title,
            icon,
            order_no,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMenu {
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub sort: i64,
    pub path: String,
    pub meta: Meta,
    pub component: String,
    pub active_name: Option<String>,
    pub keep_alive: bool,
    pub default_menu: bool,
    pub menu_level: i64,
    pub description: Option<String>,
    pub remark: Option<String>,
    #[serde(with = "date_time_format")]
    pub created_at: DateTime,
    #[serde(with = "date_time_format_option")]
    pub updated_at: Option<DateTime>,
    pub children: Option<Vec<ResponseMenu>>,
}

pub fn root_tree(nodes: &[ResponseMenu]) -> Vec<ResponseMenu> {
    nodes
        .iter()
        .filter(|item| item.parent_id == SYSTEM_PARENT_MENU_ID)
        .cloned()
        .collect()
}

pub fn child_tree(mut roots: Vec<ResponseMenu>, nodes: &[ResponseMenu]) -> Vec<ResponseMenu> {
    for root in roots.iter_mut() {
        let data: Vec<ResponseMenu> = nodes
            .iter()
            .filter(|item| item.parent_id == root.id)
            .cloned()
            .collect();
        let data = child_tree(data.clone(), nodes);
        root.children = Some(data);
    }
    roots
}

impl From<Model> for ResponseMenu {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            path: value.path,
            parent_id: value.parent_id,
            component: value.component,
            active_name: value.active_name,
            keep_alive: value.keep_alive,
            default_menu: value.default_menu,
            menu_level: value.menu_level,
            description: value.description,
            remark: value.remark,
            created_at: value.created_at,
            updated_at: value.updated_at,
            children: None,
            sort: value.sort,
            meta: Meta::new(value.title, value.icon, Some(value.sort)),
        }
    }
}
