use std::net::SocketAddrV6;

use axum::Router;
use sea_orm::DatabaseConnection;

use crate::{api::passkeys, config::Config, db};

#[derive(Clone)]
pub(crate) struct Context {
    pub(crate) database_connection: DatabaseConnection,
    pub(crate) config: Config,
}

pub async fn start_server() {
    let config = Config::load();
    let database_connection = db::connect(config.database_url.clone()).await;
    let app = Router::new()
        .merge(passkeys::registrations::router())
        .merge(passkeys::authentications::router())
        .with_state(Context {
            database_connection,
            config: config.clone(),
        });
    let address = SocketAddrV6::new(config.bind_address, config.port, 0, 0);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
