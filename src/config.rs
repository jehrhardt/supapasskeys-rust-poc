use std::{net::Ipv6Addr, str::FromStr};

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) database_url: String,
    pub(crate) relying_party_name: String,
    pub(crate) relying_party_origin: String,
    pub(crate) bind_address: Ipv6Addr,
    pub(crate) port: u16,
}

impl Config {
    pub(crate) fn load() -> Self {
        Config {
            database_url: std::env::var("DATABASE_URL").unwrap(),
            relying_party_name: std::env::var("RELYING_PARTY_NAME").unwrap(),
            relying_party_origin: std::env::var("RELYING_PARTY_ORIGIN").unwrap(),
            bind_address: std::env::var("BIND_ADDRESS").map_or(Ipv6Addr::LOCALHOST, |a| {
                Ipv6Addr::from_str(&a).expect("BIND_ADDRESS must be a valid IPv6 address")
            }),
            port: std::env::var("PORT").map_or(3000, |p| {
                p.parse::<u16>().expect("PORT must be a valid port number")
            }),
        }
    }
}
