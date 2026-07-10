use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub object: String,
    pub entry: Vec<WebhookEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEntry {
    pub id: String,
    pub time: u64,
    pub messaging: Option<Vec<WebhookMessagingEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookMessagingEvent {
    Message {
        sender: WebhookParticipant,
        recipient: WebhookParticipant,
        timestamp: u64,
        message: MessageContent,
    },
    Delivery {
        sender: WebhookParticipant,
        recipient: WebhookParticipant,
        delivery: DeliveryInfo,
    },
    Reaction {
        sender: WebhookParticipant,
        recipient: WebhookParticipant,
        timestamp: u64,
        reaction: ReactionInfo,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookParticipant {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    pub mid: String,
    pub text: Option<String>,
    pub is_echo: Option<bool>,
    pub quick_reply: Option<QuickReply>,
    pub reply_to: Option<ReplyTo>,
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReply {
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyTo {
    pub mid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryInfo {
    pub mids: Option<Vec<String>>,
    pub watermark: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionInfo {
    pub reaction: String,
    pub emoji: Option<String>,
    pub action: String,
    pub mid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub payload: Option<AttachmentPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentPayload {
    pub url: Option<String>,
    pub title: Option<String>,
    pub sticker_id: Option<u64>,
}
