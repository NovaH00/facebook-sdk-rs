use crate::auth::PageToken;
use crate::graph::{
    GraphClient,
    UserGraphClient,
    PageGraphClient,
    Method,
    GraphError,
    GraphConnection
};
use crate::api::post::PostApi;
use crate::api::conversation::ConversationApi;
use crate::api::webhook::WebhookApi;

use super::models::{Page, PageScopedUser};


#[derive(Debug, Clone)]
pub struct PageApi {
    user_graph_client: UserGraphClient,
}

impl PageApi {
    pub fn new(
        user_graph_client: &UserGraphClient
    ) -> Self {
        Self {
            user_graph_client: user_graph_client.clone(),
        }
    }

    pub async fn first_paginated_pages(
        &self,
        limit: Option<u32>
    ) -> Result<GraphConnection<Page>, GraphError> {

        let mut request = self.user_graph_client
            .request(Method::GET, "/me/accounts")
            .fields(Page::fields());

        if let Some(limit) = limit {
            request = request.limit(limit);
        };

        request
            .send::<GraphConnection<Page>>()
            .await
    }

    pub async fn next_paginated_pages(
        &self,
        limit: Option<u32>,
        current: &GraphConnection<Page>
    ) -> Result<GraphConnection<Page>, GraphError> {
        let after = current.paging
            .as_ref()
            .and_then(|p| p.cursors.as_ref())
            .and_then(|c| c.after.as_deref());

        let mut request = self.user_graph_client
            .request(Method::GET, "/me/accounts")
            .fields(Page::fields());

        if let Some(limit) = limit {
            request = request.limit(limit);
        };

        if let Some(cursor) = after {
            request = request.after(cursor);
        }

        request.send::<GraphConnection<Page>>().await
    }

    pub async fn collect_paginated_pages(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<Page>, GraphError> {
        let mut all = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut conn = self.first_paginated_pages(limit).await?;

        loop {
            if conn.data.is_empty() {
                break;
            }

            let unique: Vec<Page> = conn.data
                .drain(..)
                .filter(|p| seen.insert(p.id.clone()))
                .collect();

            if unique.is_empty() {
                break;
            }

            all.extend(unique);

            if !conn.has_more() {
                break;
            }
            conn = self.next_paginated_pages(limit, &conn).await?;
        }

        Ok(all)
    }

    pub fn get_graph_client(
        &self,
        page: &Page
    ) -> Result<PageGraphClient, GraphError> {
        let page_access_token = page
            .clone()
            .access_token
            .map(PageToken::new)
            .ok_or(GraphError::MissingAccessToken {
                origin: "get_post_api".to_string(),
                message: format!("page `{}` is missing access token", page.name)
            })?;
        Ok(GraphClient::new(page_access_token))
    }

    pub fn get_post_api(
        &self,
        page: &Page
    ) -> Result<PostApi, GraphError> {
        let page_graph_client = self.get_graph_client(page)?;

        Ok(PostApi::new(page_graph_client))
    }

    pub fn get_conversation_api(
        &self,
        page: &Page
    ) -> Result<ConversationApi, GraphError> {
        let page_graph_client = self.get_graph_client(page)?;

        Ok(ConversationApi::new(page_graph_client, &page.id))
    }

    pub fn get_webhook_api(
        &self,
        page: &Page
    ) -> Result<WebhookApi, GraphError> {
        let page_graph_client = self.get_graph_client(page)?;

        Ok(WebhookApi::new(page_graph_client, &page.id))
    }

    pub async fn get_user_info(
        &self,
        uid: impl Into<String>
    ) -> Result<PageScopedUser, GraphError> {
        self.user_graph_client
            .request(Method::GET, format!("/{}", uid.into()))
            .fields(PageScopedUser::fields())
            .send::<PageScopedUser>()
            .await
    }
}
