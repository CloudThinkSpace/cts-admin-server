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
    pub dynamic_level: Option<i32>,
    pub real_path: Option<String>,
    pub ignore_auth: Option<bool>,
    pub roles: Option<Vec<String>>,
    pub ignore_keep_alive: Option<bool>,
    pub affix: Option<bool>,
    pub icon: Option<String>,
    pub frame_src: Option<String>,
    pub transition_name: Option<String>,
    pub hide_breadcrumb: Option<bool>,
    pub carry_param: Option<bool>,
    pub hide_children_in_menu: Option<bool>,
    pub current_active_menu: Option<String>,
    pub hide_tab: Option<bool>,
    pub hide_menu: Option<bool>,
    pub order_no: Option<i64>,
    pub ignore_route: Option<bool>,
    pub hide_path_for_children: Option<bool>,
}

impl Meta {
    pub fn new(
        title: String,
        icon: Option<String>,
        order_no: Option<i64>,
        hide_menu: Option<bool>,
        ignore_keep_alive: Option<bool>,
        frame_src: Option<String>,
    ) -> Self {
        Self {
            title,
            icon,
            order_no,
            hide_menu,
            ignore_keep_alive,
            frame_src,
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
    pub path: String,
    pub meta: Meta,
    pub component: String,
    pub default_menu: bool,
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
            default_menu: value.default_menu,
            description: value.description,
            remark: value.remark,
            created_at: value.created_at,
            updated_at: value.updated_at,
            children: None,
            meta: Meta::new(
                value.title,
                value.icon,
                Some(value.sort),
                Some(value.hidden),
                Some(value.keep_alive),
                None,
            ),
        }
    }
}
