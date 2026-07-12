use thiserror::Error;

/// Errors returned by the graph module.
///
/// Covers URL construction failures, HTTP transport errors, and
/// structured Facebook API errors.
#[derive(Debug, Error)]
pub enum GraphError {
    /// Failed to parse the request URL.
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// HTTP request failed (network error, timeout, etc.).
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    /// Facebook returned an API error response.
    #[error(
        "Facebook API error: {message} \
        Code: {code:?}. Subcode {error_subcode:?}\
        Trace ID: {fbtrace_id:?} \
        Is transient: {is_transient:?}"
    )]
    Facebook {
        /// The error message from Facebook.
        message: String,
        /// The Facebook API error code.
        code: Option<u32>,
        /// The Facebook API error subcode (more specific than `code`).
        error_subcode: Option<u32>,
        /// Trace ID for debugging with Facebook support.
        fbtrace_id: Option<String>,
        /// Whether the error is transient (retry may succeed).
        is_transient: Option<bool>
    },

    /// An API struct is missing a required access token.
    #[error("{origin}: missing access token ({message})")]
    MissingAccessToken {
        /// The method that reported the error.
        origin: String,
        /// Details about which token is missing.
        message: String,
    },


}
