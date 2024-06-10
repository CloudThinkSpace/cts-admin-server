use sea_orm_migration::prelude::*;
use crate::sys::{create_tables, drop_tables, insert_data};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_tables(manager).await?;
        // 数据初始化
        insert_data(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       drop_tables(manager).await
    }
}