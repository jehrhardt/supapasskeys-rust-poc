use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use axum::{
    routing::{patch, post},
    Router,
};
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
        .route("/passkeys", post(passkeys::index::create))
        .route(
            "/passkeys/registrations/:registration_id",
            patch(passkeys::registrations::confirm),
        )
        .route(
            "/passkeys/authentications/:authentication_id",
            patch(passkeys::authentications::confirm),
        )
        .with_state(Context {
            database_connection,
            config,
        });
    let address: SocketAddr = if cfg!(debug_assertions) {
        SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3000).into()
    } else {
        SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 3000, 0, 0).into()
    };
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
