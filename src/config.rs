#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) database_url: String,
    pub(crate) relying_party_name: String,
    pub(crate) relying_party_origin: String,
}

impl Config {
    pub(crate) fn load() -> Self {
        Config {
            database_url: std::env::var("DATABASE_URL").unwrap(),
            relying_party_name: std::env::var("RELYING_PARTY_NAME").unwrap(),
            relying_party_origin: std::env::var("RELYING_PARTY_ORIGIN").unwrap(),
        }
    }
}
