//! Webhook subscription management and event deserialization.
//!
//! This module covers two sides of Facebook webhooks:
//!
//! - **Subscription management** via [`WebhookApi`] — subscribe/unsubscribe a Page
//!   to webhook fields
//! - **Event deserialization** via the [`events`] module — parse incoming webhook
//!   payloads (messages, deliveries, reactions)

mod api;
mod models;
pub mod events;

pub use api::WebhookApi;
pub use models::{WebhookField, SubscribedApp};
