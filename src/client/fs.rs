use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    base::{HashInfo, Response},
    client::Client,
    errors::Error,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileList {
    pub content: Vec<File>,
    pub header: String,
    pub provider: String,
    pub readme: String,
    pub total: u64,
    pub write: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub hash_info: Option<HashInfo>,
    pub hashinfo: String,
    pub header: Option<String>,
    pub sign: String,
    pub thumb: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub provider: Option<String>,
    pub raw_url: Option<String>,
    pub readme: Option<String>,
    // pub related: Option<String?>,
}

#[derive(Debug, Serialize)]
pub struct ListRequest {
    pub path: String,
    pub password: String,
    pub page: u64,
    pub per_page: u64,
    pub refresh: bool,
}

impl Default for ListRequest {
    fn default() -> Self {
        Self {
            path: "/".to_string(),
            password: "".to_string(),
            page: 1,
            per_page: 30,
            refresh: false,
        }
    }
}

impl From<&str> for ListRequest {
    fn from(path: &str) -> Self {
        Self {
            path: path.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AddOfflineDownloadRequest {
    pub path: String,
    pub delete_policy: String,
    pub tool: String,
    pub urls: Vec<String>,
}

impl Default for AddOfflineDownloadRequest {
    fn default() -> Self {
        Self {
            path: "/".to_string(),
            delete_policy: "delete_on_upload_succeed".to_string(),
            tool: "115 Cloud".to_string(),
            urls: vec![],
        }
    }
}

impl AddOfflineDownloadRequest {
    pub fn new(path: String, urls: Vec<String>) -> Self {
        Self {
            path,
            urls,
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub error: String,
    pub id: String,
    pub name: String,
    pub progress: f64,
    pub state: u8,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct GetFileRequest {
    pub path: String,
    pub password: String,
}

impl Default for GetFileRequest {
    fn default() -> Self {
        Self {
            path: "/".to_string(),
            password: "".to_string(),
        }
    }
}

impl From<&str> for GetFileRequest {
    fn from(path: &str) -> Self {
        Self {
            path: path.to_string(),
            ..Default::default()
        }
    }
}

impl Client {
    pub async fn get_file(&self, req: GetFileRequest) -> Result<File, Error> {
        let resp: Response<File> = self
            .client
            .post(format!("{}/api/fs/get", self.host))
            .header("Authorization", self.token.as_deref().unwrap_or_default())
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        resp.data.ok_or(Error::Message(resp.message))
    }

    pub async fn list(&self, req: ListRequest) -> Result<FileList, Error> {
        // let resp = self
        //     .client
        //     .post(format!("{}/api/fs/list", self.host))
        //     .header("Authorization", self.token.as_deref().unwrap_or_default())
        //     .json(&req)
        //     .send()
        //     .await?
        //     .bytes()
        //     .await?;

        // println!("{:?}", String::from_utf8(resp.to_vec()));
        let resp: Response<FileList> = self
            .client
            .post(format!("{}/api/fs/list", self.host))
            .header("Authorization", self.token.as_deref().unwrap_or_default())
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        resp.data.ok_or(Error::Message(resp.message))
    }

    pub async fn add_offline_download(
        &self,
        req: AddOfflineDownloadRequest,
    ) -> Result<Vec<Task>, Error> {
        let resp: Response<TaskList> = self
            .client
            .post(format!("{}/api/fs/add_offline_download", self.host))
            .header("Authorization", self.token.as_deref().unwrap_or_default())
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        resp.data
            .map(|t| t.tasks)
            .ok_or(Error::Message(resp.message))
    }
}
