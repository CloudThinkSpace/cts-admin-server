use sea_orm_migration::prelude::*;

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

impl SysMenu {
    pub async fn create_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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

    pub async fn drop_table(manager: &SchemaManager<'_>)-> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysMenu::Table).if_exists().to_owned()).await
    }
}