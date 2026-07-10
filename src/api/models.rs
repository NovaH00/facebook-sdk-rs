use serde::{Serialize, Deserialize};

/// A Facebook user or page participant.
///
/// Represents a person or entity involved in a conversation, message, or page interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// The Facebook-scoped ID of the participant.
    pub id: String,
    /// The display name of the participant.
    pub name: String,
    /// The participant's email (if available and permitted by scope).
    pub email: Option<String>,
}

/// Deserialization helper for Facebook's nested `{ data: [...] }` participant format.
///
/// Facebook often wraps arrays in a `data` envelope:
/// ```json
/// { "data": [{ "id": "...", "name": "..." }] }
/// ```
/// This function unwraps that envelope during deserialization.
pub fn deserialize_participants<'de, D>(deserializer: D) -> Result<Vec<Participant>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper { data: Vec<Participant> }
    Wrapper::deserialize(deserializer).map(|w| w.data)
}
