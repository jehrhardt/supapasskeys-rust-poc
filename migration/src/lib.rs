pub use sea_orm_migration::prelude::*;

mod m20240127_164136_create_schema_supapasskeys;
mod m20240127_165001_create_table_passkeys;
mod m20240127_190631_create_table_registrations;
mod m20240127_191420_create_table_authentications;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240127_164136_create_schema_supapasskeys::Migration),
            Box::new(m20240127_165001_create_table_passkeys::Migration),
            Box::new(m20240127_190631_create_table_registrations::Migration),
            Box::new(m20240127_191420_create_table_authentications::Migration),
        ]
    }
}
