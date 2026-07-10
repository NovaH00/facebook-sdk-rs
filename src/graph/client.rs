
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


#[derive(Debug, Clone)]
pub struct GraphClient<O, L> {
    access_token: AccessToken<O, L>,
    http_client: Client,
}

impl<O: Clone, L: Clone> GraphClient<O, L> {
    pub fn new(access_token: AccessToken<O, L>) -> Self {
        Self {
            access_token,
            http_client: Client::new()
        }
    }

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

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.graph_base_url = base_url.into();
        self
    }

    pub fn version(mut self, version: GraphVersion) -> Self {
        self.version = version;
        self
    }

    pub fn fields<const N: usize>(mut self, fields: [&'static str; N]) -> Self {
        self.query_fields = Some(Fields::from(fields));
        self
    }

    pub fn query<const N: usize>(mut self, query_params: [(&'static str, &str); N]) -> Self {
        self.query_params = Some(QueryParams::from(query_params));
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.query_params = Some(
            self.query_params.unwrap_or_default()
            .insert("limit", limit.to_string())
        );
        self
    }

    pub fn after(mut self, cursor: &str) -> Self {
        self.query_params = Some(
            self.query_params.unwrap_or_default()
                .insert("after", cursor)
        );
        self
    }

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
            // Try to parse Facebook error JSON
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

pub type UserGraphClient = GraphClient<token_owner::User, token_lifetime::Long>;
pub type PageGraphClient = GraphClient<token_owner::Page, token_lifetime::Long>;
