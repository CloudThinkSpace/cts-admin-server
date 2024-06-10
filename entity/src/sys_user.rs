//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub phone: String,
    pub email: Option<String>,
    pub status: i32,
    pub remark: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub tenant_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_tenant::Entity",
        from = "Column::TenantId",
        to = "super::sys_tenant::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SysTenant,
}

impl Related<super::sys_tenant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysTenant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}