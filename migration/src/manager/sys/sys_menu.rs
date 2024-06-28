use chrono::Local;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

use crate::TableOperation;

#[derive(DeriveIden)]
pub enum SysMenu {
    Table,
    // 编号
    Id,
    // 菜单名称
    Name,
    // 父编号
    ParentId,
    // 排序
    Sort,
    // 路由
    Path,
    // 是否隐藏,0不隐藏，1隐藏
    Hidden,
    // 对应前端文件路径
    Component,
    // 附加属性
    ActiveName,
    // 附加属性
    KeepAlive,
    // 标题
    Title,
    // 图标
    Icon,
    // 默认菜单
    DefaultMenu,
    // 菜单等级
    MenuLevel,
    // 附加属性
    CloseTab,
    // 描述
    Description,
    // 备注
    Remark,
    // 创建时间
    CreatedAt,
    // 更新时间
    UpdatedAt,
    // 删除时间
    DeletedAt,
}

impl TableOperation for SysMenu {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysMenu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysMenu::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysMenu::Name).string().not_null())
                    .col(ColumnDef::new(SysMenu::ParentId).string().not_null())
                    .col(ColumnDef::new(SysMenu::Sort).big_integer().not_null())
                    .col(ColumnDef::new(SysMenu::Path).string().not_null())
                    .col(ColumnDef::new(SysMenu::Hidden).integer().not_null().default(0))
                    .col(ColumnDef::new(SysMenu::Component).string().not_null())
                    .col(ColumnDef::new(SysMenu::ActiveName).string())
                    .col(ColumnDef::new(SysMenu::KeepAlive).integer().not_null().default(0))
                    .col(ColumnDef::new(SysMenu::Title).string().not_null())
                    .col(ColumnDef::new(SysMenu::Icon).string())
                    .col(ColumnDef::new(SysMenu::DefaultMenu).integer().not_null().default(0))
                    .col(ColumnDef::new(SysMenu::MenuLevel).big_integer().not_null().default(0))
                    .col(ColumnDef::new(SysMenu::CloseTab).integer().not_null().default(0))
                    .col(ColumnDef::new(SysMenu::Description).string())
                    .col(ColumnDef::new(SysMenu::Remark).string())
                    .col(ColumnDef::new(SysMenu::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(SysMenu::UpdatedAt).timestamp())
                    .col(ColumnDef::new(SysMenu::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn create_index(&self, _manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        Ok(())
    }

    async fn drop_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysMenu::Table).if_exists().to_owned()).await
    }

    async fn insert_data(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let data = create_data();

        for item in data.into_iter() {
            let stmt_user = Statement::from_sql_and_values(
                manager.get_database_backend(),
                "
        INSERT INTO sys_menu
        (id, name, parent_id, sort, path,Component, Title, Default_Menu,Created_At)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        ",
                item,
            );
            db.execute(stmt_user).await?;
        }


        Ok(())
    }
}

fn create_data() -> Vec<[Value; 9]> {
    let mut result = Vec::new();
    // 生成时间戳
    let now = Local::now().naive_local();
    // 用户菜单数据
    let data_user: [Value; 9] = [
        "1".into(),
        "用户管理".into(),
        "".into(),
        1.into(),
        "user/manager".into(),
        "view/user.vue".into(),
        "用户管理".into(),
        1.into(),
        now.into()
    ];
    // 角色菜单数据
    let data_role: [Value; 9] = [
        "2".into(),
        "角色管理".into(),
        "".into(),
        2.into(),
        "role/manager".into(),
        "view/role.vue".into(),
        "角色管理".into(),
        1.into(),
        now.into()
    ];
    // 菜单数据
    let data_menu: [Value; 9] = [
        "3".into(),
        "菜单管理".into(),
        "".into(),
        3.into(),
        "menu/manager".into(),
        "view/menu.vue".into(),
        "菜单管理".into(),
        1.into(),
        now.into()
    ];
    // Api数据
    let data_api: [Value; 9] = [
        "4".into(),
        "Api管理".into(),
        "".into(),
        4.into(),
        "menu/manager".into(),
        "view/menu.vue".into(),
        "Api管理".into(),
        1.into(),
        now.into()
    ];
    // 租户数据
    let data_tenant: [Value; 9] = [
        "5".into(),
        "租户管理".into(),
        "".into(),
        5.into(),
        "tenant/manager".into(),
        "view/tenant.vue".into(),
        "租户管理".into(),
        1.into(),
        now.into()
    ];
    // 采集数据
    let data_collect: [Value; 9] = [
        "6".into(),
        "采集管理".into(),
        "".into(),
        6.into(),
        "collect/manager".into(),
        "view/collect.vue".into(),
        "采集管理".into(),
        0.into(),
        now.into()
    ];
    // 项目数据
    let data_project: [Value; 9] = [
        "7".into(),
        "项目管理".into(),
        "6".into(),
        7.into(),
        "project/manager".into(),
        "view/project.vue".into(),
        "项目管理".into(),
        0.into(),
        now.into()
    ];
    result.push(data_user);
    result.push(data_role);
    result.push(data_menu);
    result.push(data_api);
    result.push(data_tenant);
    result.push(data_collect);
    result.push(data_project);

    result
}