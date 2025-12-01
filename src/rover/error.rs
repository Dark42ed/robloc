use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error")]
    HttpError(#[from] reqwest::Error),
    #[error("api error")]
    RoverError(RoverError),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoverError {
    pub error_code: String,
    pub message: String,
    pub detail: Option<serde_json::Value>,
    pub context: Option<serde_json::Value>,
}
