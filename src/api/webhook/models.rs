use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum WebhookField {
    #[strum(serialize = "messages")]
    Messages,
    #[strum(serialize = "message_deliveries")]
    MessageDeliveries,
    #[strum(serialize = "message_reactions")]
    MessageReactions,
    #[strum(serialize = "messaging_postbacks")]
    MessagingPostbacks,
    #[strum(serialize = "feed")]
    Feed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscribedApp {
    pub category: String,
    pub link: Option<String>,
    pub name: String,
    pub id: String,
}
