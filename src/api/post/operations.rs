use std::future::Future;

use crate::auth::token_lifetime;
use crate::graph::{GraphClient, GraphError, Method};
use crate::api::post::Post;

/// Trait for cross-cutting post operations (like, unlike, delete, get).
///
/// Implemented by [`PostApi`](crate::api::post::PostApi) and [`PageApi`](crate::api::page::PageApi),
/// allowing you to perform post operations with either a page-scoped or user-scoped
/// Graph client.
///
/// # Example
///
/// ```rust,no_run
/// use facebook_sdk_rs::api::post::PostOperations;
///
/// fn process_post(api: &(impl PostOperations + Sync)) {
///     api.like_post("123456789");
/// }
/// ```
pub trait PostOperations {
    /// The owner type of the underlying Graph client (User or Page).
    type Owner: Clone + Send;

    /// Returns a reference to the underlying Graph client.
    fn graph_client(&self) -> &GraphClient<Self::Owner, token_lifetime::Long>;

    /// Likes the given post.
    ///
    /// Calls `POST /{post_id}/likes`.
    fn like_post(&self, post_id: &str) -> impl Future<Output = Result<(), GraphError>> + Send
    where
        Self: Sync,
    {
        async move {
            self.graph_client()
                .request(Method::POST, format!("/{}/likes", post_id))
                .send::<serde_json::Value>()
                .await?;
            Ok(())
        }
    }

    /// Removes the like from the given post.
    ///
    /// Calls `DELETE /{post_id}/likes`.
    fn unlike_post(&self, post_id: &str) -> impl Future<Output = Result<(), GraphError>> + Send
    where
        Self: Sync,
    {
        async move {
            self.graph_client()
                .request(Method::DELETE, format!("/{}/likes", post_id))
                .send::<serde_json::Value>()
                .await?;
            Ok(())
        }
    }

    /// Deletes the given post.
    ///
    /// Calls `DELETE /{post_id}`.
    fn delete_post(&self, post_id: &str) -> impl Future<Output = Result<(), GraphError>> + Send
    where
        Self: Sync,
    {
        async move {
            self.graph_client()
                .request(Method::DELETE, format!("/{}", post_id))
                .send::<serde_json::Value>()
                .await?;
            Ok(())
        }
    }

    /// Fetches a single post by ID.
    ///
    /// Calls `GET /{post_id}` with the fields from [`Post::fields()`].
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
