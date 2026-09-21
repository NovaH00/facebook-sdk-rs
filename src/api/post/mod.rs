//! Page Post management.
//!
//! [`PostApi`] provides paginated access to a Page's posts via `GET /me/posts`
//! and can create new posts via [`PostApi::create_post`].
//!
//! The [`PostOperations`] trait adds like, unlike, delete, and get operations
//! that work across both [`PostApi`] and [`crate::api::page::PageApi`].
//!
//! [`CreatePostResponse`] holds the ID returned by Facebook after a successful
//! post creation.
//!
//! [`PostMedia`] defines photos with optional captions to attach to a post.

mod api;
mod models;
mod operations;

pub use api::PostApi;
pub use models::{Post, CreatePostResponse, PostMedia};
pub use operations::PostOperations;
