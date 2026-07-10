use crate::graph::{
    PageGraphClient,
    GraphConnection,
    GraphError,
    Method
};
use crate::api::models::Participant;

use super::models::{Message, MessagingType};
use super::schemas::SendMessageResponse;


/// High-level API for reading and sending messages in a Messenger conversation.
///
/// Provides paginated access to message history and the ability to send
/// replies via the Messenger Send API.
///
/// # Example
///
/// ```rust,no_run
/// # use facebook_sdk_rs::api::message::{MessageApi, MessagingType};
/// # use facebook_sdk_rs::graph::PageGraphClient;
/// # use facebook_sdk_rs::api::models::Participant;
/// # let client: PageGraphClient = unimplemented!();
/// # let recipient = Participant { id: "123".into(), name: "User".into(), email: None };
/// let msg_api = MessageApi::new(client, "conversation_id", recipient);
///
/// let messages = msg_api.collect_paginated_messages(None).await.unwrap();
/// let response = msg_api.send_message("Hello!", MessagingType::Response).await.unwrap();
/// println!("Sent message ID: {}", response.message_id);
/// ```
#[derive(Debug, Clone)]
pub struct MessageApi {
    page_graph_client: PageGraphClient,
    conversation_id: String,
    recipient: Participant
}

impl MessageApi {
    /// Creates a new `MessageApi`.
    ///
    /// * `page_graph_client` — A Graph client with a Page access token
    /// * `conversation_id` — The Messenger conversation ID
    /// * `recipient` — The non-Page participant (the person you're messaging)
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

    /// Fetches the first page of messages in the conversation.
    ///
    /// Calls `GET /{conversation_id}/messages`.
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

    /// Fetches the next page of messages using cursor pagination.
    pub async fn next_paginated_messages (
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

    /// Fetches all messages in the conversation, handling pagination automatically.
    ///
    /// Deduplicates results by message ID.
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

    /// Sends a text message reply in the conversation.
    ///
    /// Calls `POST /me/messages` with the recipient, message text, and messaging type.
    /// Uses `serde_json::json!()` for proper JSON escaping of the message text.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::Facebook`] if the message cannot be sent (e.g.,
    /// outside the 24-hour window without a valid message tag).
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
