use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashInfo {
    pub sha1: Option<String>,
}
