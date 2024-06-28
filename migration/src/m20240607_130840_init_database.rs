use sea_orm_migration::prelude::*;
use crate::manager::{create_indices, create_tables, drop_tables, insert_data};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建数据表
        create_tables(manager).await?;
        // 创建索引
        create_indices(manager).await?;
        // 数据初始化
        insert_data(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_tables(manager).await?;
        Ok(())
    }
}