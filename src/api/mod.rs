//! High-level domain APIs for Facebook Graph API resources.
//!
//! This module provides typed wrappers around common Facebook resources:
//!
//! - **User** — Get the current user's profile
//! - **Page** — List managed Pages, bridge to Page-scoped APIs
//! - **Post** — List, like, unlike, and delete Page posts
//! - **Conversation** — List and read Messenger conversations
//! - **Message** — Read messages from a conversation and send replies
//! - **Webhook** — Manage webhook subscriptions and deserialize incoming events
//!
//! # Service Chain
//!
//! The APIs follow a layered ownership pattern:
//!
//! ```text
//! UserApi  ──→  PageApi  ──→  PostApi
//!                       ├──→  ConversationApi  ──→  MessageApi
//!                       └──→  WebhookApi
//! ```
//!
//! Each level holds the token type appropriate for its scope (user token for
//! user/profile ops, page token for page-scoped ops).

pub mod user;
pub mod page;
pub mod post;
pub mod conversation;
pub mod message;
pub mod models;
pub mod webhook;
