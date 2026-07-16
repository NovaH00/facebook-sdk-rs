use crate::graph::{
    PageGraphClient,
    GraphConnection,
    GraphError,
    Method
};

use super::models::{WebhookField, SubscribedApp};

#[derive(Debug, Clone)]
pub struct WebhookApi {
    page_graph_client: PageGraphClient,
}

impl WebhookApi {
    pub fn new(page_graph_client: PageGraphClient) -> Self {
        Self { page_graph_client }
    }

    pub async fn subscribe(
        &self,
        fields: &[WebhookField]
    ) -> Result<(), GraphError> {
        let fields_str = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");


        self.page_graph_client
            .request(Method::POST, "/me/subscribed_apps")
            .query([("subscribed_fields", &fields_str)])
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    /// Unsubscribes the Page from the specified webhook fields.
    ///
    /// Calls `DELETE /{page_id}/subscribed_apps?subscribed_fields=...`.
    pub async fn unsubscribe(&self) -> Result<(), GraphError> {
        self.page_graph_client
            .request(Method::DELETE, "/me/subscribed_apps")
            .send::<serde_json::Value>()
            .await?;

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<SubscribedApp>, GraphError> {
        let conn = self.page_graph_client
            .request(Method::GET, "/me/subscribed_apps")
            .send::<GraphConnection<SubscribedApp>>()
            .await?;

        Ok(conn.data)
    }
}
