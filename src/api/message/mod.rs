//! Messenger Messages.
//!
//! [`MessageApi`] provides paginated access to a conversation's messages
//! and the ability to send replies via the Messenger Send API.

mod api;
mod models;
mod schemas;

pub use api::MessageApi;
pub use models::{
    Message,
    MessagingType,
    AttachmentData,
    ImageData,
    VideoData,
};
pub use schemas::SendMessageResponse;
