//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
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
    pub default_menu: bool,
    pub menu_level: i64,
    pub close_tab: bool,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_role_menu::Entity")]
    SysRoleMenu,
}

impl Related<super::sys_role_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRoleMenu.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
