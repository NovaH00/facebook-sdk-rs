use std::future::Future;

use crate::auth::token_lifetime;
use crate::graph::{GraphClient, GraphError, Method};
use crate::api::post::Post;

pub trait PostOperations {
    type Owner: Clone + Send;

    fn graph_client(&self) -> &GraphClient<Self::Owner, token_lifetime::Long>;

    fn like_post(&self, post: &Post) -> impl Future<Output = Result<(), GraphError>> + Send
    where
        Self: Sync,
    {
        async move {
            self.graph_client()
                .request(Method::POST, format!("/{}/likes", post.id))
                .send::<serde_json::Value>()
                .await?;
            Ok(())
        }
    }

    fn unlike_post(&self, post: &Post) -> impl Future<Output = Result<(), GraphError>> + Send
    where
        Self: Sync,
    {
        async move {
            self.graph_client()
                .request(Method::DELETE, format!("/{}/likes", post.id))
                .send::<serde_json::Value>()
                .await?;
            Ok(())
        }
    }

    fn delete_post(&self, post: &Post) -> impl Future<Output = Result<(), GraphError>> + Send
    where
        Self: Sync,
    {
        async move {
            self.graph_client()
                .request(Method::DELETE, format!("/{}", post.id))
                .send::<serde_json::Value>()
                .await?;
            Ok(())
        }
    }

    fn get_post(
        &self,
        id: impl Into<String>,
    ) -> impl Future<Output = Result<Post, GraphError>> + Send
    where
        Self: Sync,
    {
        let id = id.into();
        async move {
            self.graph_client()
                .request(Method::GET, id)
                .fields(Post::fields())
                .send()
                .await
        }
    }
}


