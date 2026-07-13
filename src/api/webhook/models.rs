use serde::{Deserialize, Serialize, Deserializer, Serializer};
use strum_macros::{Display, EnumString};

/// A webhook field that a Page can subscribe to.
///
/// Pass a slice of these to [`WebhookApi::subscribe`](super::WebhookApi::subscribe)
/// to receive real-time notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum WebhookField {
    /// Receive messages sent to the Page via Messenger.
    #[strum(serialize = "messages")]
    Messages,
    /// Receive delivery receipts for messages sent by the Page.
    #[strum(serialize = "message_deliveries")]
    MessageDeliveries,
    /// Receive reaction notifications when users react to Page messages.
    #[strum(serialize = "message_reactions")]
    MessageReactions,
    /// Receive postback callbacks from Messenger buttons.
    #[strum(serialize = "messaging_postbacks")]
    MessagingPostbacks,
    /// Receive feed updates (posts, reactions, shares).
    #[strum(serialize = "feed")]
    Feed,
}

impl Serialize for WebhookField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WebhookField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        <Self as std::str::FromStr>::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// An app installed on a Facebook Page.
///
/// Returned by [`WebhookApi::list`](super::WebhookApi::list).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SubscribedApp {
    /// The app's category (e.g. "Business").
    pub category: String,
    /// The app's website URL.
    pub link: Option<String>,
    /// The app's display name.
    pub name: String,
    /// The app's Facebook ID.
    pub id: String,
}
