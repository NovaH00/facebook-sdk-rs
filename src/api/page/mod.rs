//! Facebook Page management and Page-scoped API bridges.
//!
//! [`PageApi`] lists the authenticated user's managed Pages and provides
//! factory methods for Post, Conversation, and Webhook APIs that require
//! a Page-scoped token.

mod api;
mod models;

pub use api::PageApi;
pub use models::{Page, PageScopedUser};
