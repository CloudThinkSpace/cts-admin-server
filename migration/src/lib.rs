pub use sea_orm_migration::prelude::*;

mod m20240607_130840_init_database;
mod m20250523_070322_create_projectlayer_table;
mod manager;

trait TableOperation {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>;
    async fn create_index(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>;
    async fn drop_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>;
    async fn insert_data(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr>;
}

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240607_130840_init_database::Migration),
            Box::new(m20250523_070322_create_projectlayer_table::Migration),
        ]
    }
}
