//! Graph API client, request builder, pagination, and error handling.
//!
//! This module provides the low-level HTTP client for communicating with the
//! Facebook Graph API:
//!
//! - [`GraphClient<O, L>`] — Typed HTTP client with an embedded access token
//! - [`GraphRequestBuilder`] — Builder for constructing API requests with fields, pagination, etc.
//! - [`GraphConnection<T>`] — Paginated response wrapper with cursor-based paging
//! - [`GraphError`] — Comprehensive error type for API failures
//! - [`UserGraphClient`] / [`PageGraphClient`] — Convenience type aliases
//!
//! # Type safety
//!
//! `GraphClient` uses phantom-type parameters to track the token owner:
//!
//! - `UserGraphClient = GraphClient<token_owner::User, token_lifetime::Long>`
//! - `PageGraphClient = GraphClient<token_owner::Page, token_lifetime::Long>`
//!
//! This prevents mixing up user and page tokens at compile time.

mod models;
mod client;
mod errors;
mod schemas;

pub use client::{
    GraphClient,
    GraphRequestBuilder,
    UserGraphClient,
    PageGraphClient
};
pub use reqwest::Method;
pub use errors::GraphError;
pub use models::{
    Fields,
    QueryParams,
    GraphVersion,
    GraphCursor,
    GraphPaging,
    GraphConnection,
    GRAPH_BASE_URL
};
