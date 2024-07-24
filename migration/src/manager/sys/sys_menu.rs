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
                    .col(
                        ColumnDef::new(SysMenu::Hidden)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(SysMenu::Component).string().not_null())
                    .col(ColumnDef::new(SysMenu::ActiveName).string())
                    .col(
                        ColumnDef::new(SysMenu::KeepAlive)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(SysMenu::Title).string().not_null())
                    .col(ColumnDef::new(SysMenu::Icon).string())
                    .col(
                        ColumnDef::new(SysMenu::DefaultMenu)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(SysMenu::MenuLevel)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(SysMenu::CloseTab)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
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
            .drop_table(Table::drop().table(SysMenu::Table).if_exists().to_owned())
            .await
    }

    async fn insert_data(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let data = create_data();

        for item in data.into_iter() {
            let stmt_user = Statement::from_sql_and_values(
                manager.get_database_backend(),
                "
        INSERT INTO sys_menu
        (id, name, icon, parent_id, sort, path, Component, Title, Default_Menu,Created_At)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        ",
                item,
            );
            db.execute(stmt_user).await?;
        }

        Ok(())
    }
}

fn create_data() -> Vec<[Value; 10]> {
    let mut result = Vec::new();
    // 生成时间戳
    let now = Local::now().naive_local();
    // 首页菜单数据
    let data_home: [Value; 10] = [
        "0".into(),
        "home".into(),
        "ic:round-house".into(),
        "".into(),
        0.into(),
        "/dashboard".into(),
        "/dashboard/analysis/index.vue".into(),
        "首页".into(),
        true.into(),
        now.into(),
    ];

    // 用户菜单数据
    let data_user: [Value; 10] = [
        "1".into(),
        "user".into(),
        "ic:baseline-supervisor-account".into(),
        "".into(),
        1.into(),
        "/user/index".into(),
        "/sys/user/UserTable.vue".into(),
        "用户管理".into(),
        true.into(),
        now.into(),
    ];
    // 角色菜单数据
    let data_role: [Value; 10] = [
        "2".into(),
        "role".into(),
        "ic:round-join-left".into(),
        "".into(),
        2.into(),
        "/role/index".into(),
        "/sys/role/RoleTable.vue".into(),
        "角色管理".into(),
        true.into(),
        now.into(),
    ];
    // 菜单数据
    let data_menu: [Value; 10] = [
        "3".into(),
        "menu".into(),
        "ic:round-view-list".into(),
        "".into(),
        3.into(),
        "/menu/index".into(),
        "/demo/table/FormTable.vue".into(),
        "菜单管理".into(),
        true.into(),
        now.into(),
    ];
    // Api数据
    let data_api: [Value; 10] = [
        "4".into(),
        "api".into(),
        "ic:sharp-playlist-add-check-circle".into(),
        "".into(),
        4.into(),
        "/api/index".into(),
        "/sys/api/ApiTable.vue".into(),
        "Api管理".into(),
        true.into(),
        now.into(),
    ];
    // 租户数据
    let data_tenant: [Value; 10] = [
        "5".into(),
        "tenant".into(),
        "ic:baseline-groups".into(),
        "".into(),
        5.into(),
        "/tenant/index".into(),
        "/sys/tenant/TenantTable.vue".into(),
        "租户管理".into(),
        true.into(),
        now.into(),
    ];
    // 采集数据
    let data_collect: [Value; 10] = [
        "6".into(),
        "collection".into(),
        "ic:baseline-api".into(),
        "".into(),
        6.into(),
        "collect/index".into(),
        "/demo/table/FormTable.vue".into(),
        "采集管理".into(),
        false.into(),
        now.into(),
    ];
    // 项目数据
    let data_project: [Value; 10] = [
        "7".into(),
        "project".into(),
        "ic:baseline-article".into(),
        "6".into(),
        7.into(),
        "project/manager".into(),
        "/demo/table/FormTable.vue".into(),
        "项目管理".into(),
        false.into(),
        now.into(),
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
