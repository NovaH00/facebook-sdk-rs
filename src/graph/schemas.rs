use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct FacebookErrorResponse {
    pub error: FacebookErrorDetail,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FacebookErrorDetail {
    pub message: String,
    pub code: u32,
    #[serde(default)]
    pub error_subcode: Option<u32>,
    #[serde(default)]
    pub fbtrace_id: Option<String>,
    #[serde(default)]
    pub is_transient: Option<bool>,
}
