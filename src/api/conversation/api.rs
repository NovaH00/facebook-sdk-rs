
use crate::graph::{
    PageGraphClient,
    GraphConnection,
    GraphError,
    Method
};
use crate::api::message::MessageApi;
use super::models::{Conversation};


#[derive(Debug, Clone)]
pub struct ConversationApi {
    page_graph_client: PageGraphClient,
    page_id: String,
}

impl ConversationApi {
    pub fn new(
        page_graph_client: PageGraphClient,
        page_id: impl Into<String>
    ) -> Self {
        Self {
            page_graph_client,
            page_id: page_id.into()
        }
    }

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
