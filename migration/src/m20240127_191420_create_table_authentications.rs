use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "create table supapasskeys.authentications (
                id uuid not null default gen_random_uuid(),
                user_id uuid not null,
                state jsonb not null,
                confirmed_at timestamp with time zone null,
                created_at timestamp with time zone not null default now(),
                updated_at timestamp with time zone not null default now(),
                constraint authentications_pkey primary key (id)
            )",
        )
        .await?;
        db.execute_unprepared( "create index if not exists authentications_user_id_idx on supapasskeys.authentications using btree (user_id)").await?;

        Ok(())
    }
}
