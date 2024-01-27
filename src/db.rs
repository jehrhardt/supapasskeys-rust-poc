use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub(crate) async fn connect(database_url: String) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .set_schema_search_path("supapasskeys");
    if cfg!(debug_assertions) {
        opt.sqlx_logging(true);
    }
    Database::connect(opt)
        .await
        .expect("can't connect to database")
}
