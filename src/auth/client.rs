use url::Url;

use crate::graph::{GraphVersion, GRAPH_BASE_URL};

use super::{
    models::{
        AppPermission,
        AppAuthType,
        AccessTokenInfo,
        AccessToken,
        ShortLivedUserToken,
        LongLivedUserToken,
        OAUTH_BASE_URL
    },
    schemas::{
        AccessTokenResponse,
        DebugTokenResponse
    },
    errors:: AuthError
};

/// Facebook OAuth client for managing app credentials and token exchange.
///
/// `AuthClient` is the entry point for the Facebook Login flow. It holds your
/// Facebook app's credentials and provides methods for:
///
/// - Generating OAuth consent URLs
/// - Exchanging authorization codes for access tokens
/// - Extending short-lived tokens to long-lived tokens
/// - Debugging/inspecting any token
///
/// # Example
///
/// ```rust,no_run
/// use facebook_sdk_rs::auth::{AuthClient, AppPermission};
///
/// let app = AuthClient::new(
///     "123456789",
///     "app-secret",
///     "https://myapp.com/callback",
/// );
///
/// let url = app.get_oauth_url(
///     "random-state",
///     &[AppPermission::PagesShowList],
///     None,
/// ).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct AuthClient {
    app_id: String,
    app_secret: String,
    redirect_url: String,
    version: GraphVersion,
    oauth_base_url: String,
    graph_base_url: String,
    http_client: reqwest::Client
}

impl AuthClient {
    /// Creates a new `AuthClient` from Facebook app credentials.
    ///
    /// By default, the latest stable Graph API version is used. Use [`set_version`](Self::set_version)
    /// to override it.
    pub fn new(
        app_id: impl Into<String>,
        app_secret: impl Into<String>,
        redirect_url: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            redirect_url: redirect_url.into(),
            version: GraphVersion::default(),
            oauth_base_url: OAUTH_BASE_URL.into(),
            graph_base_url: GRAPH_BASE_URL.into(),
            http_client: reqwest::Client::new()
        }
    }

    /// Overrides the Graph API version used for all requests from this client.
    ///
    /// Defaults to the latest stable version. Call this immediately after `new()` to
    /// pin a specific version.
    pub fn set_version(mut self, version: GraphVersion) -> Self {
        self.version = version;
        self
    }

    /// Builds the Facebook OAuth consent URL to redirect users to.
    ///
    /// The user will see the Facebook Login dialog requesting the specified
    /// [`AppPermission`] scopes. After they authorize, Facebook redirects to
    /// the configured `redirect_url` with an authorization `code` that can be
    /// exchanged via [`login`](Self::login).
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::Url`] if the built URL is invalid.
    ///
    /// # Parameters
    ///
    /// * `state` — An opaque value the server uses to prevent CSRF attacks
    /// * `scope` — List of permissions to request
    /// * `auth_type` — Optional re-auth behavior (e.g. `Reauthenticate`)
    pub fn get_oauth_url(
        &self,
        state: impl Into<String>,
        scope: &[AppPermission],
        auth_type: Option<AppAuthType>
    ) -> Result<String, AuthError> {
        let mut params = vec![
            ("client_id", self.app_id.clone()),
            ("redirect_uri", self.redirect_url.clone()),
            ("state", state.into()),
            ("response_type", "code".into()),
        ];

        let mut permissions = scope
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        permissions.sort();
        params.push(("scope", permissions.join(",")));

        if let Some(auth_type) = auth_type {
            params.push(("auth_type", auth_type.to_string()))
        }

        let mut url = Url::parse(&format!(
            "{}/{}/dialog/oauth",
            self.oauth_base_url,
            self.version,
        ))?;

        url.query_pairs_mut().extend_pairs(params);

        Ok(url.to_string())
    }

    async fn exchange_short_lived_token(
        &self,
        code: impl Into<String>
    ) -> Result<ShortLivedUserToken, AuthError> {
        let params = [
            ("client_id", self.app_id.clone()),
            ("client_secret", self.app_secret.clone()),
            ("redirect_uri", self.redirect_url.clone()),
            ("code", code.into())
        ];

        let response = self
            .http_client
            .get(format!("{}/{}/oauth/access_token", self.graph_base_url, self.version))
            .query(&params)
            .send()
            .await?
            .error_for_status()?;

        let data = response.json::<AccessTokenResponse>().await?;

        data.access_token
            .map(AccessToken::new)
            .ok_or(AuthError::MissingAccessToken)
    }

    async fn exchange_long_lived_token(
        &self,
        access_token: ShortLivedUserToken
    ) -> Result<LongLivedUserToken, AuthError> {
        let params = [
            ("grant_type", "fb_exchange_token".to_string()),
            ("client_id", self.app_id.clone()),
            ("client_secret", self.app_secret.clone()),
            ("fb_exchange_token", access_token.as_str().to_string())
        ];

        let response = self
            .http_client
            .get(format!("{}/{}/oauth/access_token", self.graph_base_url, self.version))
            .query(&params)
            .send()
            .await?
            .error_for_status()?;

        let data = response.json::<AccessTokenResponse>().await?;
        data.access_token
            .map(AccessToken::new)
            .ok_or(AuthError::MissingAccessToken)
    }

    /// Exchanges an authorization code for a long-lived user access token.
    ///
    /// This is a two-step process:
    /// 1. Exchange the authorization code for a short-lived token
    /// 2. Exchange the short-lived token for a long-lived token (60 days)
    ///
    /// Call this with the `code` query parameter received at your redirect URL
    /// after the user authorizes your app.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::Request`] if the HTTP request fails, or
    /// [`AuthError::MissingAccessToken`] if Facebook's response is missing the token.
    pub async fn login(
        &self,
        code: impl Into<String>
    ) -> Result<LongLivedUserToken, AuthError> {
        let short_lived_token = self.exchange_short_lived_token(code).await?;

        self.exchange_long_lived_token(short_lived_token).await
    }

    /// Inspects a Facebook access token via the `/debug_token` endpoint.
    ///
    /// Returns metadata including the token's owner, scopes, expiry, and validity.
    /// Works with any token type (user, page, app).
    ///
    /// Uses the app access token (formatted as `{app_id}|{app_secret}`) for authorization.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::Request`] if the HTTP request fails.
    pub async fn debug_token<O, L>(
        &self,
        token: AccessToken<O, L>
    ) -> Result<AccessTokenInfo, AuthError> {
        let params = [
            ("input_token", token.as_str().to_string()),
            ("access_token", format!("{}|{}", self.app_id, self.app_secret))
        ];

        let response = self
            .http_client
            .get(format!("{}/{}/debug_token", self.graph_base_url, self.version))
            .query(&params)
            .send()
            .await?
            .error_for_status()?;

        let token_info = response.json::<DebugTokenResponse>().await?.data;
        Ok(token_info)
    }
}
