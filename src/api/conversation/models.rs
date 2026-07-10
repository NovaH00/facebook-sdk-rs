use serde::{Serialize, Deserialize};

use crate::api::models::{Participant, deserialize_participants};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub message_count: u32,
    pub unread_count: u32,

    #[serde(deserialize_with = "deserialize_participants")]
    pub participants: Vec<Participant>,
}

impl Conversation {
    pub fn fields() -> [&'static str; 4] {
        ["id", "message_count", "unread_count", "participants"]
    }

    pub fn recipient(&self, page_id: &str) -> Option<&Participant> {
        self.participants.iter().find(|p| p.id != page_id)
    }
}
