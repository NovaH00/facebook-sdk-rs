use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Facebook response did not contain an access_token")]
    MissingAccessToken,
}
