use serde::Deserialize;

use super::models::AccessTokenInfo;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AccessTokenResponse {
    pub access_token: Option<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DebugTokenResponse {
    pub data: AccessTokenInfo,
}
