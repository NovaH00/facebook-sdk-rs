//! OAuth 2.0 authentication and typed access tokens.
//!
//! This module provides the full Facebook Login flow:
//!
//! - [`AuthClient`] — Manages app credentials, generates OAuth URLs, exchanges codes for tokens
//! - [`AccessToken<O, L>`] — Phantom-typed token that tracks owner (User/Page) and lifetime (Short/Long)
//! - [`LongLivedUserToken`] / [`ShortLivedUserToken`] — Convenience type aliases
//! - [`AppPermission`] — Facebook OAuth permission scopes
//! - [`AccessTokenInfo`] — Token introspection results from `/debug_token`

mod client;
mod models;
mod schemas;
mod errors;

pub use client::AuthClient;
pub use models::{
    AppPermission,
    AppAuthType,
    AccessToken,
    token_owner,
    token_lifetime,
    ShortLivedUserToken,
    LongLivedUserToken,
    PageToken,
    AccessTokenInfo,
    OAUTH_BASE_URL
};
pub use errors::AuthError;
