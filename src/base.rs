use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}
