use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{base, errors::Error};

use super::Client;

#[derive(Debug, Deserialize)]
struct AuthResponse {
    token: String,
}

impl Client {
    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), Error> {
        let url = format!("{}/api/auth/login/hash", self.host);
        let req = json!({
            "username": username,
            "password": Self::sha256(password),
        });
        let resp: base::Response<AuthResponse> = self
            .client
            .post(url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        if resp.code != 200 {
            return Err(Error::Message(resp.message));
        }
        self.token = Some(resp.data.unwrap().token);
        dbg!(&self.token);
        Ok(())
    }

    fn sha256(str: &str) -> String {
        let value = Sha256::digest(format!("{}-https://github.com/alist-org/alist", str));
        format!("{:x}", value)
    }
}
