use crate::auth::PageToken;
use crate::graph::{
    GraphClient,
    UserGraphClient,
    PageGraphClient,
    Method,
    GraphError,
    GraphConnection
};

use super::models::{Page, PageScopedUser};

/// High-level API for listing Pages managed by the authenticated user.
///
/// `PageApi` lists the user's managed Pages and provides a helper
/// to extract a [`PageGraphClient`] from a Page's access token.
///
/// # Example
///
/// ```rust,no_run
/// # use facebook_sdk_rs::api::page::PageApi;
/// # use facebook_sdk_rs::graph::UserGraphClient;
/// # let user_client: UserGraphClient = unimplemented!();
/// let page_api = PageApi::new(&user_client);
///
/// let pages = page_api.collect_paginated_pages(None).await.unwrap();
/// for page in &pages {
///     let client = page_api.get_graph_client(page).unwrap();
///     // use client for page-scoped API calls
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PageApi {
    user_graph_client: UserGraphClient,
}

impl PageApi {
    /// Creates a new `PageApi` from a user-scoped Graph client.
    pub fn new(
        user_graph_client: &UserGraphClient
    ) -> Self {
        Self {
            user_graph_client: user_graph_client.clone(),
        }
    }

    /// Fetches the first page of the user's managed Pages.
    ///
    /// Calls `GET /me/accounts`. Use [`next_paginated_pages`](Self::next_paginated_pages)
    /// to fetch subsequent pages and [`collect_paginated_pages`](Self::collect_paginated_pages)
    /// to fetch all pages at once.
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

    /// Fetches the next page of the user's managed Pages using cursor pagination.
    ///
    /// Pass the previous response as `current`. The `after` cursor is extracted
    /// from `current.paging.cursors`.
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

    /// Fetches all of the user's managed Pages, handling pagination automatically.
    ///
    /// Deduplicates results by Page ID. Pass `limit` to control page size (per
    /// API request), or `None` for the default.
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

    /// Extracts a [`PageGraphClient`] from a Page (requires the Page's access token).
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::MissingAccessToken`] if the Page has no `access_token`.
    pub fn get_graph_client(
        &self,
        page: &Page
    ) -> Result<PageGraphClient, GraphError> {
        let page_access_token = page
            .clone()
            .access_token
            .map(PageToken::new)
            .ok_or(GraphError::MissingAccessToken {
                origin: "get_graph_client".to_string(),
                message: format!("page `{}` is missing access token", page.name)
            })?;
        Ok(GraphClient::new(page_access_token))
    }

    /// Looks up a user by their Page-scoped ID (PSID).
    ///
    /// Calls `GET /{psid}` with the user Graph client, returning Page-scoped
    /// user information.
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
