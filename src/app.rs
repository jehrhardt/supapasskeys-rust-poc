use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use axum::{
    routing::{patch, post},
    Router,
};

use crate::api;

pub async fn start_server() {
    let app = Router::new()
        .route("/passkeys", post(api::passkeys::index::create))
        .route(
            "/passkeys/registrations/:registration_id",
            patch(api::passkeys::registrations::confirm),
        )
        .route(
            "/passkeys/authentications/:authentication_id",
            patch(api::passkeys::authentications::confirm),
        );
    let address: SocketAddr = if cfg!(debug_assertions) {
        SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3000).into()
    } else {
        SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 3000, 0, 0).into()
    };
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
