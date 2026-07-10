
use crate::graph::{
    PageGraphClient,
    GraphConnection,
    GraphError,
    Method
};
use crate::api::models::Participant;

use super::models::{Message, MessagingType};
use super::schemas::SendMessageResponse;


#[derive(Debug, Clone)]
pub struct MessageApi {
    page_graph_client: PageGraphClient,
    conversation_id: String,
    recipient: Participant
}

impl MessageApi {
    pub fn new(
        page_graph_client: PageGraphClient,
        conversation_id: impl Into<String>,
        recipient: Participant
    ) -> Self {
        Self {
            page_graph_client,
            conversation_id: conversation_id.into(),
            recipient
        }
    }

    pub async fn first_paginated_messages(
        &self,
        limit: Option<u32>,
    ) -> Result<GraphConnection<Message>, GraphError> {
        let mut request = self.page_graph_client
            .request(Method::GET, format!("/{}/messages", self.conversation_id))
            .fields(Message::fields());

        if let Some(limit) = limit {
            request = request.limit(limit);
        }

        request.send().await
    }

    pub async fn next_paginated_messages(
        &self,
        limit: Option<u32>,
        current: &GraphConnection<Message>,
    ) -> Result<GraphConnection<Message>, GraphError> {
        let after = current.paging
            .as_ref()
            .and_then(|p| p.cursors.as_ref())
            .and_then(|c| c.after.as_deref());

        let mut request = self.page_graph_client
            .request(Method::GET, format!("/{}/messages", self.conversation_id))
            .fields(Message::fields());

        if let Some(limit) = limit {
            request = request.limit(limit);
        }

        if let Some(cursor) = after {
            request = request.after(cursor);
        }

        request.send().await
    }

    pub async fn collect_paginated_messages(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<Message>, GraphError> {
        let mut all = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut conn = self.first_paginated_messages(limit).await?;

        loop {
            if conn.data.is_empty() {
                break;
            }

            let unique: Vec<Message> = conn.data
                .drain(..)
                .filter(|m| seen.insert(m.id.clone()))
                .collect();

            if unique.is_empty() {
                break;
            }

            all.extend(unique);

            if !conn.has_more() {
                break;
            }

            conn = self.next_paginated_messages(limit, &conn).await?;
        }

        Ok(all)
    }

    pub async fn send_message(
        &self,
        message: &str,
        messaging_type: MessagingType,
    ) -> Result<SendMessageResponse, GraphError> {
        let recipient_json = serde_json::json!({"id": &self.recipient.id}).to_string();
        let message_json = serde_json::json!({"text": message}).to_string();
        let messaging_type_str = messaging_type.to_string();

        self.page_graph_client
            .request(Method::POST, "/me/messages")
            .query([
                ("recipient", &recipient_json),
                ("message", &message_json),
                ("messaging_type", &messaging_type_str),
            ])
            .send()
            .await
    }
}
