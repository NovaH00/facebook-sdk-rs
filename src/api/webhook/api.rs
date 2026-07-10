
use crate::graph::{
    PageGraphClient,
    GraphError,
    Method
};

use super::models::{WebhookField, SubscribedApp};


#[derive(Debug, Clone)]
pub struct WebhookApi {
    page_graph_client: PageGraphClient,
    page_id: String,
}

impl WebhookApi {
    pub fn new(page_graph_client: PageGraphClient, page_id: impl Into<String>) -> Self {
        Self { page_graph_client, page_id: page_id.into() }
    }

    /// POST /{page_id}/subscribed_apps?subscribed_fields=messages,...
    pub async fn subscribe(&self, fields: &[WebhookField]) -> Result<(), GraphError> {
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.page_graph_client
            .request(Method::POST, format!("/{}/subscribed_apps", self.page_id))
            .query([("subscribed_fields", &fields_str)])
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    /// DELETE /{page_id}/subscribed_apps?subscribed_fields=messages
    pub async fn unsubscribe(&self, fields: &[WebhookField]) -> Result<(), GraphError> {
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.page_graph_client
            .request(Method::DELETE, format!("/{}/subscribed_apps", self.page_id))
            .query([("subscribed_fields", &fields_str)])
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    /// DELETE /{page_id}/subscribed_apps (no fields = unsubscribe all)
    pub async fn unsubscribe_all(&self) -> Result<(), GraphError> {
        self.page_graph_client
            .request(Method::DELETE, format!("/{}/subscribed_apps", self.page_id))
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    /// GET /{page_id}/subscribed_apps — returns list of installed apps
    pub async fn list(&self) -> Result<Vec<SubscribedApp>, GraphError> {
        #[derive(serde::Deserialize)]
        struct Response { data: Vec<SubscribedApp> }
        let resp = self.page_graph_client
            .request(Method::GET, format!("/{}/subscribed_apps", self.page_id))
            .send::<Response>()
            .await?;

        Ok(resp.data)
    }
}
