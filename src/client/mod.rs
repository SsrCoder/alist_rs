pub mod admin;
pub mod auth;
pub mod fs;
pub mod me;

pub struct Client {
    host: String,
    client: reqwest::Client,
    token: Option<String>,
}

impl Client {
    pub fn new(host: String) -> Self {
        Self {
            host,
            client: reqwest::Client::new(),
            token: None,
        }
    }
}
