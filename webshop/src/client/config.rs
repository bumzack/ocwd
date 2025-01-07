use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub basic_auth_key: String,
    pub database_url: String,
}

pub fn get_client(keep_alive: u64) -> reqwest::Result<Client> {
    Client::builder()
        .tcp_keepalive(Some(std::time::Duration::from_secs(keep_alive)))
        .build()
}
