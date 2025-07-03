use sea_orm::{ConnectionTrait, DbErr, Statement};
use sea_orm_migration::SchemaManager;

use crate::manager::sys::sys_menu::SysMenu;
use crate::manager::sys::sys_role::SysRole;
use crate::{ColumnDef, DeriveIden, ForeignKey, Table, TableOperation, Value};

#[derive(DeriveIden)]
pub enum SysRoleMenu {
    Table,
    Id,
    // 角色Id
    RoleId,
    // 菜单Id
    MenuId,
}

impl TableOperation for SysRoleMenu {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysRoleMenu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysRoleMenu::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysRoleMenu::RoleId).string().not_null())
                    .col(ColumnDef::new(SysRoleMenu::MenuId).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn create_index(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        // 创建 角色外键
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("Fk_role-menu_role_id")
                    .from(SysRoleMenu::Table, SysRoleMenu::RoleId)
                    .to(SysRole::Table, SysRole::Id)
                    .to_owned(),
            )
            .await?;

        // 创建 角色外键
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("Fk_role-menu_menu_id")
                    .from(SysRoleMenu::Table, SysRoleMenu::MenuId)
                    .to(SysMenu::Table, SysMenu::Id)
                    .to_owned(),
            )
            .await
    }

    async fn drop_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(SysRoleMenu::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn insert_data(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let data = create_data();

        for item in data.into_iter() {
            let stmt_user = Statement::from_sql_and_values(
                manager.get_database_backend(),
                "
        INSERT INTO sys_role_menu
        (id, role_id, menu_id)
        VALUES ($1,$2,$3)
        ",
                item,
            );
            db.execute(stmt_user).await?;
        }

        Ok(())
    }
}

fn create_data() -> Vec<[Value; 3]> {
    let mut result = Vec::new();

    // 首页数据
    let data_home: [Value; 3] = [
        uuid::Uuid::new_v4().to_string().into(),
        "1".into(),
        "0".into(),
    ];

    // 用户菜单数据
    let data_user: [Value; 3] = [
        uuid::Uuid::new_v4().to_string().into(),
        "1".into(),
        "1".into(),
    ];
    // 角色菜单数据
    let data_role: [Value; 3] = [
        uuid::Uuid::new_v4().to_string().into(),
        "1".into(),
        "2".into(),
    ];
    // 菜单数据
    let data_menu: [Value; 3] = [
        uuid::Uuid::new_v4().to_string().into(),
        "1".into(),
        "3".into(),
    ];
    // Api数据
    let data_api: [Value; 3] = [
        uuid::Uuid::new_v4().to_string().into(),
        "1".into(),
        "4".into(),
    ];
    // 租户数据
    let data_tenant: [Value; 3] = [
        uuid::Uuid::new_v4().to_string().into(),
        "1".into(),
        "5".into(),
    ];

    // 采集数据
    let data_collect: [Value; 3] = [
        uuid::Uuid::new_v4().to_string().into(),
        "1".into(),
        "6".into(),
    ];

    // 项目数据
    let data_project: [Value; 3] = [
        uuid::Uuid::new_v4().to_string().into(),
        "1".into(),
        "7".into(),
    ];

    result.push(data_home);
    result.push(data_user);
    result.push(data_role);
    result.push(data_menu);
    result.push(data_api);
    result.push(data_tenant);
    result.push(data_collect);
    result.push(data_project);

    result
}
