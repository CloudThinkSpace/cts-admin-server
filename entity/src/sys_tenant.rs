//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_tenant")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub enabled: i32,
    pub r#type: i32,
    pub description: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_role::Entity")]
    SysRole,
    #[sea_orm(has_many = "super::sys_user::Entity")]
    SysUser,
}

impl Related<super::sys_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRole.def()
    }
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
