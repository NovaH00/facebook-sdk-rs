use thiserror::Error;

/// Errors returned by the auth module.
#[derive(Debug, Error)]
pub enum AuthError {
    /// Failed to parse a URL (e.g., malformed redirect URI).
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    /// HTTP request to Facebook failed.
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    /// Facebook's response did not contain an access token.
    ///
    /// This typically means the authorization code was invalid or expired.
    #[error("Facebook response did not contain an access_token")]
    MissingAccessToken,
}
