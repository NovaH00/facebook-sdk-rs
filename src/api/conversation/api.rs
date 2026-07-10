use crate::graph::{
    PageGraphClient,
    GraphConnection,
    GraphError,
    Method
};
use crate::api::message::MessageApi;
use super::models::{Conversation};


/// High-level API for reading a Page's Messenger conversations.
///
/// Provides paginated access to the Page's conversations and a factory
/// for creating [`MessageApi`] instances.
///
/// # Example
///
/// ```rust,no_run
/// # use facebook_sdk_rs::api::conversation::ConversationApi;
/// # use facebook_sdk_rs::graph::PageGraphClient;
/// # let client: PageGraphClient = unimplemented!();
/// let conv_api = ConversationApi::new(client, "123456789");
/// let conversations = conv_api.collect_paginated_conversations(None).await.unwrap();
/// for conv in &conversations {
///     let msg_api = conv_api.get_message_api(conv).unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ConversationApi {
    page_graph_client: PageGraphClient,
    page_id: String,
}

impl ConversationApi {
    /// Creates a new `ConversationApi`.
    ///
    /// * `page_graph_client` — A Graph client with a Page access token
    /// * `page_id` — The Page's Facebook ID (used to resolve recipients)
    pub fn new(
        page_graph_client: PageGraphClient,
        page_id: impl Into<String>
    ) -> Self {
        Self {
            page_graph_client,
            page_id: page_id.into()
        }
    }

    /// Fetches the first page of the Page's conversations.
    ///
    /// Calls `GET /me/conversations`.
    pub async fn first_paginated_conversations(
        &self,
        limit: Option<u32>,
    ) -> Result<GraphConnection<Conversation>, GraphError> {
        let mut request = self.page_graph_client
            .request(Method::GET, "/me/conversations")
            .fields(Conversation::fields());

        if let Some(limit) = limit {
            request = request.limit(limit);
        }

        request.send().await
    }

    /// Fetches the next page of conversations using cursor pagination.
    pub async fn next_paginated_conversations(
        &self,
        limit: Option<u32>,
        current: &GraphConnection<Conversation>,
    ) -> Result<GraphConnection<Conversation>, GraphError> {
        let after = current.paging
            .as_ref()
            .and_then(|p| p.cursors.as_ref())
            .and_then(|c| c.after.as_deref());

        let mut request = self.page_graph_client
            .request(Method::GET, "/me/conversations")
            .fields(Conversation::fields());

        if let Some(limit) = limit {
            request = request.limit(limit);
        }

        if let Some(cursor) = after {
            request = request.after(cursor);
        }

        request.send().await
    }

    /// Fetches all conversations, handling pagination automatically.
    ///
    /// Deduplicates results by conversation ID.
    pub async fn collect_paginated_conversations(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<Conversation>, GraphError> {
        let mut all = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut conn = self.first_paginated_conversations(limit).await?;

        loop {
            if conn.data.is_empty() { break; }

            let unique: Vec<Conversation> = conn.data
                .drain(..)
                .filter(|c| seen.insert(c.id.clone()))
                .collect();

            if unique.is_empty() { break; }
            all.extend(unique);

            if !conn.has_more() { break; }
            conn = self.next_paginated_conversations(limit, &conn).await?;
        }

        Ok(all)
    }

    /// Fetches a single conversation by ID.
    ///
    /// Calls `GET /{conversation_id}`.
    pub async fn get_conversation(
        &self,
        conversation_id: &str,
    ) -> Result<Conversation, GraphError> {
        self.page_graph_client
            .request(Method::GET, conversation_id)
            .fields(Conversation::fields())
            .send()
            .await
    }

    /// Creates a [`MessageApi`] for the given conversation.
    ///
    /// Resolves the conversation's recipient (the non-Page participant)
    /// automatically. Returns [`GraphError::MissingRecipient`] if the
    /// conversation has no non-Page participant.
    pub fn get_message_api(
        &self,
        conversation: &Conversation
    ) -> Result<MessageApi, GraphError> {
        let recipient = conversation
            .recipient(&self.page_id)
            .ok_or(GraphError::MissingRecipient {
                origin: "get_message_api".into(),
                conversation_id: conversation.id.clone(),
                existing_participants: conversation
                    .participants
                    .clone()
                    .into_iter()
                    .map(|pa| pa.name)
                    .collect::<Vec<String>>()
                    .join(", ")
            })?;

        Ok(MessageApi::new(
            self.page_graph_client.clone(),
            &conversation.id,
            recipient.clone()
        ))
    }
}
