use serde::{Serialize, Deserialize};

/// The top-level payload received from a Facebook webhook POST request.
///
/// # Example
///
/// ```json
/// {
///   "object": "page",
///   "entry": [{
///     "id": "123456789",
///     "time": 1458692752478,
///     "messaging": [{
///       "sender": { "id": "user_psid" },
///       "recipient": { "id": "page_id" },
///       "message": { "mid": "...", "text": "hello" }
///     }]
///   }]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct WebhookPayload {
    /// The object type (always `"page"` for Page webhooks).
    pub object: String,
    /// One or more event entries, each belonging to a page.
    pub entry: Vec<WebhookEntry>,
}

/// A single entry in the webhook payload, corresponding to one page.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct WebhookEntry {
    /// The Page ID that triggered the event.
    pub id: String,
    /// The time the event occurred (epoch milliseconds).
    pub time: u64,
    /// Messenger messaging events (only present for Messenger webhooks).
    pub messaging: Option<Vec<WebhookMessagingEvent>>,
}

/// A messaging event within a webhook entry.
///
/// Each event can be one of three types: a message, a delivery receipt,
/// or a reaction. Use `match` to handle each case.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum WebhookMessagingEvent {
    /// A message was sent or received.
    Message {
        sender: WebhookParticipant,
        recipient: WebhookParticipant,
        timestamp: u64,
        message: MessageContent,
    },
    /// A delivery receipt for a message sent by the page.
    Delivery {
        sender: WebhookParticipant,
        recipient: WebhookParticipant,
        delivery: DeliveryInfo,
    },
    /// A reaction was added or removed on a message.
    Reaction {
        sender: WebhookParticipant,
        recipient: WebhookParticipant,
        timestamp: u64,
        reaction: ReactionInfo,
    },
}

/// A participant in a messaging event (sender or recipient).
///
/// Only includes the PSID (Page-Scoped ID). Use
/// [`crate::api::models::Participant`] for the richer participant type
/// that includes name and email.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct WebhookParticipant {
    /// The Page-scoped user ID (PSID).
    pub id: String,
}

/// Content of a received or sent message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MessageContent {
    /// The message ID.
    pub mid: String,
    /// The text content of the message (absent for attachment-only messages).
    pub text: Option<String>,
    /// Whether this message is an echo (sent by the page, not the user).
    pub is_echo: Option<bool>,
    /// The quick reply payload, if the message was sent via a quick reply button.
    pub quick_reply: Option<QuickReply>,
    /// Reference to the message this one is replying to.
    pub reply_to: Option<ReplyTo>,
    /// Attachments in the message.
    pub attachments: Option<Vec<Attachment>>,
}

/// A quick reply payload from a message button interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct QuickReply {
    /// The developer-defined payload string from the quick reply button.
    pub payload: Option<String>,
}

/// Reference to a message being replied to.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReplyTo {
    /// The message ID being replied to.
    pub mid: Option<String>,
}

/// Delivery receipt information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DeliveryInfo {
    /// Message IDs that were delivered (may be absent for older clients).
    pub mids: Option<Vec<String>>,
    /// All messages with timestamps before this were delivered.
    pub watermark: u64,
}

/// Reaction information (when a user reacts to a message).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReactionInfo {
    /// The reaction type (e.g. `"smile"`, `"like"`, `"love"`, `"angry"`, etc.).
    pub reaction: String,
    /// The emoji used for the reaction.
    pub emoji: Option<String>,
    /// Whether the reaction was added (`"react"`) or removed (`"unreact"`).
    pub action: String,
    /// The message ID that the reaction applies to.
    pub mid: String,
}

/// An attachment in a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Attachment {
    /// The attachment type (e.g. `"image"`, `"video"`, `"audio"`, `"file"`, `"sticker"`).
    #[serde(rename = "type")]
    pub attachment_type: String,
    /// The attachment payload (type-specific data).
    pub payload: Option<AttachmentPayload>,
}

/// Payload data for a message attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AttachmentPayload {
    /// The URL of the attachment.
    pub url: Option<String>,
    /// The title of the attachment (for shared content).
    pub title: Option<String>,
    /// The sticker ID (for sticker attachments).
    pub sticker_id: Option<u64>,
}
