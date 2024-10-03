use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("message: {0}")]
    Message(String),
}
