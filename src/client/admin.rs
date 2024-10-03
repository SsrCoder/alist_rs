use crate::{base::Response, errors::Error};

use super::{fs::Task, Client};

impl Client {
    pub async fn offline_download_undone(&self) -> Result<Vec<Task>, Error> {
        let resp: Response<Vec<Task>> = self
            .client
            .get(format!(
                "{}/api/admin/task/offline_download/undone",
                self.host
            ))
            .header("Authorization", self.token.as_deref().unwrap_or_default())
            .send()
            .await?
            .json()
            .await?;
        resp.data.ok_or(Error::Message(resp.message))
    }
}
