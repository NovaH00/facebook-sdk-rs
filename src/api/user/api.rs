use crate::graph::{
    UserGraphClient,
    GraphError,
    Method
};
use crate::api::page::PageApi;

use super::models::User;

/// High-level API for the authenticated Facebook user.
///
/// Provides access to the user's profile and bridges to Page-scoped APIs.
///
/// # Example
///
/// ```rust,no_run
/// # async fn _test() {
/// use facebook_sdk_rs::api::user::UserApi;
/// use facebook_sdk_rs::graph::{GraphClient, UserGraphClient};
/// # use facebook_sdk_rs::auth::LongLivedUserToken;
///
/// # let token: LongLivedUserToken = unimplemented!();
/// let user_api = UserApi::new(UserGraphClient::new(token));
/// let user = user_api.me().await.unwrap();
/// println!("Hello, {}!", user.name);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct UserApi {
    user_graph_client: UserGraphClient,
}

impl UserApi {
    /// Creates a new `UserApi` from a long-lived user access token.
    pub fn new(
        user_graph_client: UserGraphClient
    ) -> Self {
        Self {
            user_graph_client
        }
    }

    /// Fetches the authenticated user's profile.
    ///
    /// Calls `GET /me` with the fields returned by [`User::fields()`].
    ///
    /// # Errors
    ///
    /// Returns [`GraphError`] if the request fails or the token is invalid.
    pub async fn me(
        &self,
    ) -> Result<User, GraphError> {
        self.user_graph_client
            .request(Method::GET, "/me")
            .fields(User::fields())
            .send::<User>()
            .await
    }

    /// Revokes all permissions for the authenticated user's access token.
    ///
    /// Calls `DELETE /me/permissions`. After calling this, the token is
    /// invalidated and the user must re-authorize your app.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError`] if the request fails or the token cannot be
    /// revoked.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # async fn _test() {
    /// # use facebook_sdk_rs::api::user::UserApi;
    /// # let user_api: UserApi = unimplemented!();
    /// user_api.revoke_permissions().await.unwrap();
    /// # }
    /// ```
    pub async fn revoke_permissions(&self) -> Result<(), GraphError> {
        self.user_graph_client
            .request(Method::DELETE, "/me/permissions")
            .send::<serde_json::Value>()
            .await?;
        Ok(())
    }

    /// Creates a [`PageApi`] for listing and managing the user's Pages.
    ///
    /// The returned `PageApi` uses the same user access token to fetch
    /// the user's managed Pages via `GET /me/accounts`.
    pub fn get_page_api(
        &self,
    ) -> PageApi {
        PageApi::new(&self.user_graph_client)
    }
}
