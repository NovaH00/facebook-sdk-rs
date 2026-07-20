use crate::graph::{
    PageGraphClient,
    GraphConnection,
    GraphError,
    Method
};
use super::models::Post;


/// High-level API for reading a Page's posts.
///
/// Provides paginated access to `GET /me/posts` with automatic deduplication.
/// For post operations (like, unlike, delete), use the [`PostOperations`](super::PostOperations)
/// trait which is implemented by this type.
///
/// # Example
///
/// ```rust,no_run
/// # async fn _test() {
/// # use facebook_sdk_rs::api::post::PostApi;
/// # use facebook_sdk_rs::graph::PageGraphClient;
/// # let client: PageGraphClient = unimplemented!();
/// let post_api = PostApi::new(client);
/// let posts = post_api.collect_paginated_posts(None).await.unwrap();
/// for post in &posts {
///     println!("{}", post.message.as_deref().unwrap_or("(no text)"));
/// }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PostApi {
    page_graph_client: PageGraphClient
}

impl PostApi {
    /// Creates a new `PostApi` from a Page-scoped Graph client.
    pub fn new(
        page_graph_client: PageGraphClient
    ) -> Self {
        Self {
            page_graph_client
        }
    }

    /// Fetches the first page of the Page's posts.
    ///
    /// Calls `GET /me/posts`. Use [`next_paginated_posts`](Self::next_paginated_posts)
    /// to fetch subsequent pages.
    pub async fn first_paginated_posts(
        &self,
        limit: Option<u32>
    ) -> Result<GraphConnection<Post>, GraphError> {

        let mut request = self.page_graph_client
            .request(Method::GET, "/me/posts")
            .fields(Post::fields());


        if let Some(limit) = limit {
            request = request.limit(limit);
        };

        request
            .send::<GraphConnection<Post>>()
            .await
    }

    /// Fetches the next page of the Page's posts using cursor pagination.
    pub async fn next_paginated_posts(
        &self,
        limit: Option<u32>,
        current: &GraphConnection<Post>
    ) -> Result<GraphConnection<Post>, GraphError> {
        let after = current.paging
            .as_ref()
            .and_then(|p| p.cursors.as_ref())
            .and_then(|c| c.after.as_deref());

        let mut request = self.page_graph_client
            .request(Method::GET, "/me/posts")
            .fields(Post::fields());


        if let Some(limit) = limit {
            request = request.limit(limit);
        };

        if let Some(cursor) = after {
            request = request.after(cursor);
        }

        request.send::<GraphConnection<Post>>().await
    }

    /// Fetches all of the Page's posts, handling pagination automatically.
    ///
    /// Deduplicates results by post ID.
    pub async fn collect_paginated_posts(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<Post>, GraphError> {
        let mut all = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut conn = self.first_paginated_posts(limit).await?;

        loop {
            if conn.data.is_empty() {
                break;
            }

            let unique: Vec<Post> = conn.data
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

            conn = self.next_paginated_posts(limit, &conn).await?;
        }

        Ok(all)
    }
}
