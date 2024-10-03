use serde::Deserialize;

use crate::{base::Response, errors::Error};

use super::Client;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub base_path: String,
    pub disabled: bool,
    pub id: u64,
    pub otp: bool,
    pub password: String,
    pub permission: u64,
    pub role: u64,
    pub sso_id: String,
    pub username: String,
}

impl Client {
    pub async fn me(&self) -> Result<UserInfo, Error> {
        let url = format!("{}/api/me", self.host);
        let resp: Response<UserInfo> = self
            .client
            .get(url)
            .header("Authorization", self.token.as_deref().unwrap_or_default())
            .send()
            .await?
            .json()
            .await?;
        resp.data.ok_or(Error::Message("me not found".to_string()))
    }
}
