
use crate::graph::{
    PageGraphClient,
    GraphConnection,
    GraphError,
    Method
};
use super::models::Post;


#[derive(Debug, Clone)]
pub struct PostApi {
    page_graph_client: PageGraphClient
}

impl PostApi {
    pub fn new(
        page_graph_client: PageGraphClient
    ) -> Self {
        Self {
            page_graph_client
        }
    }

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
