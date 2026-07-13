use serde::{Deserialize, Serialize};

/// Response from the Messenger Send API after successfully sending a message.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SendMessageResponse {
    /// The ID of the sent message.
    pub message_id: String,
    /// The ID of the recipient that received the message.
    pub recipient_id: String
}
