use serde::Deserialize;

use super::models::AccessTokenInfo;

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DebugTokenResponse {
    pub data: AccessTokenInfo,
}
