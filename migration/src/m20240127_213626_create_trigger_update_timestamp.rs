use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "create or replace function supapasskeys.update_timestamp()
            returns trigger as $$
            begin
                new.updated_at := now();
                return new;
            end;
            $$ language plpgsql",
        )
        .await?;
        db.execute_unprepared(
            "create trigger passkeys_update_timestamp before
            update on supapasskeys.passkeys for each row
            execute function supapasskeys.update_timestamp()",
        )
        .await?;
        db.execute_unprepared(
            "create trigger registrations_update_timestamp before
            update on supapasskeys.registrations for each row
            execute function supapasskeys.update_timestamp()",
        )
        .await?;
        db.execute_unprepared(
            "create trigger authentications_update_timestamp before
            update on supapasskeys.authentications for each row
            execute function supapasskeys.update_timestamp()",
        )
        .await?;

        Ok(())
    }
}
