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
/// let webhook = WebhookApi::new(client, "page_id");
///
/// webhook.subscribe(&[
///     WebhookField::Messages,
///     WebhookField::MessageDeliveries,
/// ]).await.unwrap();
///
/// let apps = webhook.list().await.unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct WebhookApi {
    page_graph_client: PageGraphClient,
    page_id: String,
}

impl WebhookApi {
    /// Creates a new `WebhookApi` for the given Page.
    pub fn new(page_graph_client: PageGraphClient, page_id: impl Into<String>) -> Self {
        Self { page_graph_client, page_id: page_id.into() }
    }

    /// Subscribes the Page to the specified webhook fields.
    ///
    /// Calls `POST /{page_id}/subscribed_apps?subscribed_fields=...`.
    /// Facebook returns `{ "success": true }` on success.
    ///
    /// The Page must have installed the app first (usually done via the App Dashboard
    /// or by a Page admin granting the `pages_manage_metadata` permission).
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

    /// Unsubscribes the Page from the specified webhook fields.
    ///
    /// Calls `DELETE /{page_id}/subscribed_apps?subscribed_fields=...`.
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

    /// Unsubscribes the Page from all webhook fields.
    ///
    /// Calls `DELETE /{page_id}/subscribed_apps` without a `subscribed_fields` parameter.
    pub async fn unsubscribe_all(&self) -> Result<(), GraphError> {
        self.page_graph_client
            .request(Method::DELETE, format!("/{}/subscribed_apps", self.page_id))
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    /// Lists the apps installed on this Page.
    ///
    /// Calls `GET /{page_id}/subscribed_apps`. Returns an empty vec if no apps
    /// are installed.
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
