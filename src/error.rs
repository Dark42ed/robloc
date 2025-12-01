use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error")]
    HttpError(#[from] reqwest::Error),
    #[error("invalid http header")]
    HttpHeaderError(#[from] reqwest::header::ToStrError),
}
