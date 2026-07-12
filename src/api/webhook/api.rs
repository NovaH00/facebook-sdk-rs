use crate::graph::{
    PageGraphClient,
    GraphError,
    Method
};

use super::models::{WebhookField, SubscribedApp};

/// High-level API for managing a Page's webhook subscriptions.
///
/// Allows subscribing and unsubscribing a Page to specific webhook fields.
/// The callback URL is configured at the app level (via the Facebook App Dashboard
/// or the `/{app-id}/subscriptions` endpoint) — this API only controls which
/// Pages and fields receive events.
///
/// # Example
///
/// ```rust,no_run
/// # use facebook_sdk_rs::api::webhook::{WebhookApi, WebhookField};
/// # use facebook_sdk_rs::graph::PageGraphClient;
/// # let client: PageGraphClient = unimplemented!();
/// let webhook = WebhookApi::new(client);
///
/// webhook.subscribe("page_id", &[
///     WebhookField::Messages,
///     WebhookField::MessageDeliveries,
/// ]).await.unwrap();
///
/// let apps = webhook.list("page_id").await.unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct WebhookApi {
    page_graph_client: PageGraphClient,
}

impl WebhookApi {
    /// Creates a new `WebhookApi`.
    pub fn new(page_graph_client: PageGraphClient) -> Self {
        Self { page_graph_client }
    }

    /// Subscribes the Page to the specified webhook fields.
    ///
    /// Calls `POST /{page_id}/subscribed_apps?subscribed_fields=...`.
    pub async fn subscribe(&self, page_id: &str, fields: &[WebhookField]) -> Result<(), GraphError> {
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.page_graph_client
            .request(Method::POST, format!("/{}/subscribed_apps", page_id))
            .query([("subscribed_fields", &fields_str)])
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    /// Unsubscribes the Page from the specified webhook fields.
    ///
    /// Calls `DELETE /{page_id}/subscribed_apps?subscribed_fields=...`.
    pub async fn unsubscribe(&self, page_id: &str, fields: &[WebhookField]) -> Result<(), GraphError> {
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.page_graph_client
            .request(Method::DELETE, format!("/{}/subscribed_apps", page_id))
            .query([("subscribed_fields", &fields_str)])
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    /// Unsubscribes the Page from all webhook fields.
    ///
    /// Calls `DELETE /{page_id}/subscribed_apps`.
    pub async fn unsubscribe_all(&self, page_id: &str) -> Result<(), GraphError> {
        self.page_graph_client
            .request(Method::DELETE, format!("/{}/subscribed_apps", page_id))
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    /// Lists the apps installed on this Page.
    ///
    /// Calls `GET /{page_id}/subscribed_apps`.
    pub async fn list(&self, page_id: &str) -> Result<Vec<SubscribedApp>, GraphError> {
        #[derive(serde::Deserialize)]
        struct Response { data: Vec<SubscribedApp> }
        let resp = self.page_graph_client
            .request(Method::GET, format!("/{}/subscribed_apps", page_id))
            .send::<Response>()
            .await?;

        Ok(resp.data)
    }
}
