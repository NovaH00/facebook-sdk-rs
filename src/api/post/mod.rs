//! Page Post management.
//!
//! [`PostApi`] provides paginated access to a Page's posts via
//! `GET /me/posts`. The [`PostOperations`] trait adds like, unlike,
//! delete, and get operations that work across both [`PostApi`] and
//! [`crate::api::page::PageApi`].

mod api;
mod models;
mod operations;

pub use api::PostApi;
pub use models::Post;
pub use operations::PostOperations;
