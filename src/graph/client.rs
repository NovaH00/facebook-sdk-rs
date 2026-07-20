use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;

use crate::auth::{AccessToken, token_owner, token_lifetime};
use super::schemas::{FacebookErrorResponse};

use super::{
    models::{
        Fields,
        QueryParams,
        GraphVersion,
        GRAPH_BASE_URL
    },
    errors::GraphError
};


/// Typed HTTP client for the Facebook Graph API.
///
/// `GraphClient` wraps an access token and an HTTP client. Use the type aliases
/// [`UserGraphClient`] and [`PageGraphClient`] for the common token scopes.
///
/// # Example
///
/// ```rust,no_run
/// use facebook_sdk_rs::graph::{GraphClient, Method, UserGraphClient};
/// use facebook_sdk_rs::auth::LongLivedUserToken;
///
/// let token: LongLivedUserToken = unimplemented!();
/// let client: UserGraphClient = GraphClient::new(token);
///
/// let request = client.request(Method::GET, "/me")
///     .fields(["id", "name"]);
/// ```
#[derive(Debug, Clone)]
pub struct GraphClient<O, L> {
    access_token: AccessToken<O, L>,
    http_client: Client,
}

impl<O: Clone, L: Clone> GraphClient<O, L> {
    /// Creates a new GraphClient from an access token.
    pub fn new(access_token: AccessToken<O, L>) -> Self {
        Self {
            access_token,
            http_client: Client::new()
        }
    }

    /// Starts building a request to the given endpoint.
    ///
    /// `endpoint` is the path relative to the Graph API base URL,
    /// e.g. `"/me"`, `"/me/accounts"`, `"/{page-id}/feed"`.
    pub fn request(
        &self,
        method: Method,
        endpoint: impl Into<String>,
    ) -> GraphRequestBuilder<O, L> {

        GraphRequestBuilder::new(
            method,
            self.access_token.clone(),
            endpoint.into(),
            self.http_client.clone()
        )
    }

}


/// Builder for constructing and sending Graph API requests.
///
/// Created by [`GraphClient::request`]. Use the builder methods to
/// configure the request, then call [`send`](GraphRequestBuilder::send)
/// to execute it.
///
/// # Example
///
/// ```rust,no_run
/// # async fn _test() {
/// # use facebook_sdk_rs::graph::{GraphClient, Method, UserGraphClient};
/// # use facebook_sdk_rs::auth::LongLivedUserToken;
/// # let client: UserGraphClient = GraphClient::new(unimplemented!());
/// let response: serde_json::Value = client
///     .request(Method::GET, "/me/accounts")
///     .fields(["id", "name", "access_token"])
///     .limit(50)
///     .send()
///     .await
///     .unwrap();
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct GraphRequestBuilder<O, L> {
    method: Method,
    access_token: AccessToken<O, L>,
    graph_base_url: String,
    version: GraphVersion,
    endpoint: String,
    query_fields: Option<Fields>,
    query_params: Option<QueryParams>,
    http_client: Client,
}

impl<O, L> GraphRequestBuilder<O, L> {
    fn new(
        method: Method,
        access_token: AccessToken<O, L>,
        endpoint: String,
        http_client: Client,
    ) -> Self {
        Self {
            method,
            access_token,
            graph_base_url: GRAPH_BASE_URL.into(),
            version: GraphVersion::default(),
            endpoint,
            query_fields: None,
            query_params: None,
            http_client,
        }
    }

    /// Overrides the base URL (defaults to `https://graph.facebook.com`).
    pub fn set_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.graph_base_url = base_url.into();
        self
    }

    /// Sets the Graph API version (defaults to latest).
    pub fn set_version(mut self, version: GraphVersion) -> Self {
        self.version = version;
        self
    }

    /// Sets the `fields` query parameter for field selection.
    pub fn fields<const N: usize>(mut self, fields: [&'static str; N]) -> Self {
        self.query_fields = Some(Fields::from(fields));
        self
    }

    /// Adds raw query parameters to the request.
    ///
    /// Used internally by higher-level APIs for parameters like
    /// `recipient`, `message`, `messaging_type`, etc.
    pub fn query<const N: usize>(mut self, query_params: [(&'static str, &str); N]) -> Self {
        self.query_params = Some(QueryParams::from(query_params));
        self
    }

    /// Sets the `limit` parameter for paginated results.
    pub fn limit(mut self, limit: u32) -> Self {
        self.query_params = Some(
            self.query_params.unwrap_or_default()
            .insert("limit", limit.to_string())
        );
        self
    }

    /// Sets the `after` cursor for cursor-based pagination.
    ///
    /// Use the `after` value from the previous response's
    /// [`GraphPaging`](crate::graph::GraphPaging) cursors.
    pub fn after(mut self, cursor: &str) -> Self {
        self.query_params = Some(
            self.query_params.unwrap_or_default()
                .insert("after", cursor)
        );
        self
    }

    /// Sends the request and deserializes the response.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::UrlParseError`] if the constructed URL is invalid,
    /// [`GraphError::Request`] for HTTP/transport failures, or
    /// [`GraphError::Facebook`] if Facebook returns an API error response.
    pub async fn send<T: DeserializeOwned>(self) -> Result<T, GraphError> {
        let url = Url::parse(&format!(
            "{}/{}/{}", self.graph_base_url, self.version, self.endpoint
        ))?;

        let mut query = self.query_params.unwrap_or_default();
        query = query.insert("access_token", self.access_token.as_str());
        if let Some(fields) = self.query_fields {
            query = query.insert("fields", fields.as_slice().join(","));
        }

        let response = self.http_client
            .request(self.method, url)
            .query(&query.as_slice())
            .send()
            .await?;


        let status = response.status();
        if !status.is_success() {
            let error_body  = response
                .json::<FacebookErrorResponse>()
                .await
                .map_err(|_| {
                    GraphError::Facebook {
                        message: format!("HTTP {}: failed to parse error body", status),
                        code: Some(status.as_u16() as u32),
                        error_subcode: None,
                        fbtrace_id: None,
                        is_transient: None
                    }
                })?;

            return Err(GraphError::Facebook {
                message: error_body.error.message,
                code: Some(error_body.error.code),
                error_subcode: error_body.error.error_subcode,
                fbtrace_id: error_body.error.fbtrace_id,
                is_transient: error_body.error.is_transient
            });
        }

        Ok(response.json::<T>().await?)
    }

}

/// Graph client with a long-lived user access token.
pub type UserGraphClient = GraphClient<token_owner::User, token_lifetime::Long>;

/// Graph client with a long-lived page access token.
pub type PageGraphClient = GraphClient<token_owner::Page, token_lifetime::Long>;
