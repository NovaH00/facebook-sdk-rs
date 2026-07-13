use std::str::FromStr;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use strum_macros::{Display, EnumString};

use crate::api::models::{Participant};


/// Metadata about an image attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ImageData {
    /// The image width in pixels.
    pub width: u32,
    /// The image height in pixels.
    pub height: u32,
    /// The maximum rendering width.
    pub max_width: u32,
    /// The maximum rendering height.
    pub max_height: u32,
    /// The image URL.
    pub url: String,
    /// A preview URL for the image.
    pub preview_url: Option<String>,
    /// The image type identifier.
    pub image_type: Option<u32>,
    /// Whether the image should render as a sticker.
    pub render_as_sticker: Option<bool>,
}

/// Metadata about a video attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct VideoData {
    /// The video width in pixels.
    pub width: u32,
    /// The video height in pixels.
    pub height: u32,
    /// The video duration in seconds.
    pub length: f64,
    /// The video URL.
    pub url: String,
    /// A preview URL for the video.
    pub preview_url: Option<String>,
    /// The rotation angle of the video.
    pub rotation: Option<u32>,
    /// The video type identifier.
    pub video_type: Option<u32>,
}

/// An attachment in a message.
///
/// Facebook returns attachments in one of several shapes. This enum captures
/// the known variants (Image, Video, File) and falls back to [`Other`](AttachmentData::Other)
/// for unknown types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AttachmentData {
    /// An image attachment with metadata.
    Image {
        id: String,
        mime_type: Option<String>,
        name: Option<String>,
        image_data: ImageData,
    },
    /// A video attachment with metadata.
    Video {
        id: String,
        mime_type: Option<String>,
        name: Option<String>,
        video_data: VideoData,
    },
    /// A file attachment.
    File {
        id: String,
        mime_type: Option<String>,
        name: Option<String>,
        file_url: String,
        size: Option<u32>,
    },
    /// An unrecognized attachment type (raw JSON preserved).
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

/// A message in a Messenger conversation.
#[derive(Debug, Clone, Serialize,  Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Message {
    /// The message ID.
    pub id: String,
    /// When the message was created.
    pub created_time: DateTime<Utc>,
    /// The sender of the message.
    pub from: Participant,
    /// The recipient of the message.
    #[serde(deserialize_with = "deserialize_recipient")]
    pub to: Participant,
    /// The text content of the message.
    pub message: Option<String>,
    /// Attachments included in the message (if any).
    #[serde(default, deserialize_with = "deserialize_attachment_list")]
    pub attachments: Option<Vec<AttachmentData>>,
}

impl Message {
   /// Returns the field names available on this type for Graph API field selection.
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

/// Supported media types for attachment messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum MediaType {
    Image,
    Audio,
    Video,
    File,
}

impl AsRef<str> for MediaType {
    fn as_ref(&self) -> &'static str {
        match self {
            MediaType::Image => "image",
            MediaType::Audio => "audio",
            MediaType::Video => "video",
            MediaType::File => "file",
        }
    }
}

/// Message content to send via the Messenger Send API.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SendMessagePayload {
    /// A plain text message → `{ "text": "..." }`
    Text(String),
    /// A media attachment → `{ "attachment": { "type": "...", "payload": { "url": "...", "is_reusable": bool? } } }`
    Media {
        media_type: MediaType,
        url: String,
        is_reusable: Option<bool>,
    },
}

impl Serialize for SendMessagePayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SendMessagePayload::Text(text) => {
                #[derive(Serialize)]
                struct Helper<'a> { text: &'a str }
                Helper { text }.serialize(serializer)
            }
            SendMessagePayload::Media { media_type, url, is_reusable } => {
                #[derive(Serialize)]
                struct Payload<'a> {
                    url: &'a str,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    is_reusable: Option<bool>,
                }
                #[derive(Serialize)]
                struct Attachment<'a> {
                    #[serde(rename = "type")]
                    attachment_type: &'a str,
                    payload: Payload<'a>,
                }
                #[derive(Serialize)]
                struct Helper<'a> {
                    attachment: Attachment<'a>,
                }
                Helper {
                    attachment: Attachment {
                        attachment_type: media_type.as_ref(),
                        payload: Payload { url, is_reusable: *is_reusable },
                    },
                }
                .serialize(serializer)
            }
        }
    }
}

/// The messaging type for a message sent via the Send API.
///
/// Controls when Facebook allows the message to be delivered:
///
/// - [`Response`](MessagingType::Response) — Reply to a user's message within the 24-hour window
/// - [`Update`](MessagingType::Update) — Proactive update (e.g. order confirmation, appointment reminder)
/// - [`MessageTag`](MessagingType::MessageTag) — Tagged message that bypasses the 24-hour window (e.g. shipping notification)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum MessagingType {
    /// Reply to a received message (default, works within 24h window).
    #[strum(serialize = "RESPONSE")]
    Response,
    /// Proactive update (order confirmation, account alert, etc.).
    #[strum(serialize = "UPDATE")]
    Update,
    /// Tagged message that can bypass the 24-hour window.
    #[strum(serialize = "MESSAGE_TAG")]
    MessageTag,
}

impl Serialize for MessagingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MessagingType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        MessagingType::from_str(&s).map_err(serde::de::Error::custom)
    }
}
