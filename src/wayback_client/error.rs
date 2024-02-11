#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error occurred while fetching the CDX API: {0}")]
    FetchCdx(#[from] reqwest::Error),
    #[error("An error occurred while serializing the CDX API parameters: {0}")]
    SerdeUrlParams(#[from] serde_url_params::Error),
    #[error("An error occurred: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
