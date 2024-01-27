use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "create table supapasskeys.passkeys (
                id uuid not null default gen_random_uuid(),
                user_id uuid not null,
                key jsonb not null,
                created_at timestamp with time zone not null default now(),
                updated_at timestamp with time zone not null default now(),
                constraint passkeys_pkey primary key (id)
            )",
        )
        .await?;
        db.execute_unprepared( "create index if not exists passkeys_user_id_idx on supapasskeys.passkeys using btree (user_id)").await?;

        Ok(())
    }
}
