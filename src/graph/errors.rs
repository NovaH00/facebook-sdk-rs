use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error(
        "Facebook API error: {message} \
        Code: {code:?}. Subcode {error_subcode:?}\
        Trace ID: {fbtrace_id:?} \
        Is transient: {is_transient:?}"
    )]
    Facebook {
        message: String,
        code: Option<u32>,
        error_subcode: Option<u32>,
        fbtrace_id: Option<String>,
        is_transient: Option<bool>
    },

    #[error("{origin}: missing access token ({message})")]
    MissingAccessToken {
        origin: String,
        message: String,
    },

    #[error(
        "{origin}: missing recipient in conversation ({conversation_id}). \
         Existing participants: {existing_participants}"
    )]
    MissingRecipient {
        origin: String,
        conversation_id: String,
        existing_participants: String,
    }
}
