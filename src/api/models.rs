use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}

pub fn deserialize_participants<'de, D>(deserializer: D) -> Result<Vec<Participant>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper { data: Vec<Participant> }
    Wrapper::deserialize(deserializer).map(|w| w.data)
}
