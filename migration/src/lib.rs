pub use sea_orm_migration::prelude::*;

mod m20240127_164136_create_schema_supapasskeys;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20240127_164136_create_schema_supapasskeys::Migration,
        )]
    }
}
