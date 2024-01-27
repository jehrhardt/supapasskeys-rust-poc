use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "create table supapasskeys.registrations (
              id uuid not null default gen_random_uuid(),
              state jsonb not null,
              confirmed_at timestamp with time zone null,
              created_at timestamp with time zone not null default now(),
              updated_at timestamp with time zone not null default now(),
              constraint registrations_pkey primary key (id)
            )",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("drop table supapasskeys.registrations")
            .await?;

        Ok(())
    }
}
