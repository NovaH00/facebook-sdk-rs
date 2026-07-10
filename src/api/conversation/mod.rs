//! Messenger Conversations.
//!
//! [`ConversationApi`] provides paginated access to a Page's conversations
//! and a factory method for creating [`MessageApi`](crate::api::message::MessageApi)
//! instances scoped to a specific conversation.

mod models;
mod api;

pub use api::ConversationApi;
pub use models::{Conversation};
