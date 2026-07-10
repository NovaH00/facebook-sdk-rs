//! User profile and the entry point for Page-scoped APIs.
//!
//! [`UserApi`] is typically the first API you construct from a user access
//! token. It can fetch the user's profile and create a [`PageApi`](crate::api::page::PageApi)
//! for working with the user's managed Pages.

mod api;
mod models;

pub use api::UserApi;
pub use models::User;
