use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use strum_macros::{Display, EnumString};

use crate::api::models::{Participant};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub url: String,
    pub preview_url: Option<String>,
    pub image_type: Option<u32>,
    pub render_as_sticker: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoData {
    pub width: u32,
    pub height: u32,
    pub length: f64,
    pub url: String,
    pub preview_url: Option<String>,
    pub rotation: Option<u32>,
    pub video_type: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachmentData {
    Image {
        id: String,
        mime_type: Option<String>,
        name: Option<String>,
        image_data: ImageData,
    },
    Video {
        id: String,
        mime_type: Option<String>,
        name: Option<String>,
        video_data: VideoData,
    },
    File {
        id: String,
        mime_type: Option<String>,
        name: Option<String>,
        file_url: String,
        size: Option<u32>,
    },
    Other(serde_json::Value),
}


fn deserialize_attachment_list<'de, D>(d: D) -> Result<Option<Vec<AttachmentData>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper { data: Vec<AttachmentData> }
    Option::<Wrapper>::deserialize(d).map(|w| w.map(|w| w.data))
}

#[derive(Debug, Clone, Serialize,  Deserialize)]
pub struct Message {
    pub id: String,
    pub created_time: DateTime<Utc>,
    pub from: Participant,

    #[serde(deserialize_with = "deserialize_recipient")]
    pub to: Participant,

    pub message: Option<String>,

    #[serde(default, deserialize_with = "deserialize_attachment_list")]
    pub attachments: Option<Vec<AttachmentData>>,
}

impl Message {
   pub fn fields() -> [&'static str; 6] {
        ["id", "created_time", "from", "to", "message", "attachments"]
    }
}

fn deserialize_recipient<'de, D>(d: D) -> Result<Participant, D::Error>
where D: serde::Deserializer<'de> {
    #[derive(Deserialize)]
    struct Wrapper { data: Vec<Participant> }
    Wrapper::deserialize(d).and_then(|w| w.data.into_iter().next().ok_or_else(|| {
        serde::de::Error::custom("to.data is empty")
    }))
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum MessagingType {
    #[strum(serialize = "RESPONSE")]
    Response,
    #[strum(serialize = "UPDATE")]
    Update,
    #[strum(serialize = "MESSAGE_TAG")]
    MessageTag,
}
