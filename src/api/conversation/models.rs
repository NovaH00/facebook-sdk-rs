use serde::{Serialize, Deserialize};

use crate::api::models::{Participant, deserialize_participants};

/// A Messenger conversation between a Page and a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// The conversation ID.
    pub id: String,
    /// Total number of messages in the conversation.
    pub message_count: u32,
    /// Number of unread messages.
    pub unread_count: u32,
    /// Participants in the conversation (usually the Page + one user).
    #[serde(deserialize_with = "deserialize_participants")]
    pub participants: Vec<Participant>,
}

impl Conversation {
    /// Returns the field names available on this type for Graph API field selection.
    pub fn fields() -> [&'static str; 4] {
        ["id", "message_count", "unread_count", "participants"]
    }

    /// Returns the non-Page participant (the "other" person in the conversation).
    ///
    /// Filters out the participant whose `id` matches `page_id`, returning
    /// the conversation partner. Returns `None` if the Page is the only participant
    /// (which should not happen in normal Messenger conversations).
    pub fn recipient(&self, page_id: &str) -> Option<&Participant> {
        self.participants.iter().find(|p| p.id != page_id)
    }
}
