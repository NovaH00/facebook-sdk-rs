use std::fmt;
use serde::{Serialize, Deserialize};

/// Supported file types for the Meta Resumable Upload API.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UploadFileType {
    /// `application/pdf`
    #[serde(rename = "application/pdf")]
    Pdf,

    /// `image/jpeg`
    #[serde(rename = "image/jpeg")]
    Jpeg,

    /// `image/jpg`
    #[serde(rename = "image/jpg")]
    Jpg,

    /// `image/png`
    #[serde(rename = "image/png")]
    Png,

    /// `video/mp4`
    #[serde(rename = "video/mp4")]
    Mp4,

    /// Custom MIME type
    #[serde(untagged)]
    Custom(String),
}

impl UploadFileType {
    /// Returns the MIME type as a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pdf => "application/pdf",
            Self::Jpeg => "image/jpeg",
            Self::Jpg => "image/jpg",
            Self::Png => "image/png",
            Self::Mp4 => "video/mp4",
            Self::Custom(s) => s.as_str(),
        }
    }

    /// Infers [`UploadFileType`] from a file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        let clean = ext.trim_start_matches('.').to_ascii_lowercase();
        match clean.as_str() {
            "pdf" => Some(Self::Pdf),
            "jpeg" => Some(Self::Jpeg),
            "jpg" => Some(Self::Jpg),
            "png" => Some(Self::Png),
            "mp4" => Some(Self::Mp4),
            _ => None,
        }
    }
}

impl fmt::Display for UploadFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for UploadFileType {
    fn from(s: &str) -> Self {
        match s {
            "application/pdf" => Self::Pdf,
            "image/jpeg" => Self::Jpeg,
            "image/jpg" => Self::Jpg,
            "image/png" => Self::Png,
            "video/mp4" => Self::Mp4,
            other => Self::Custom(other.to_owned()),
        }
    }
}

impl From<String> for UploadFileType {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

/// An upload session initiated with Meta's Resumable Upload API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UploadSession {
    /// Upload session ID returned by Meta (for example: `"upload:123456"`).
    pub id: String,
}

/// Deserializes a byte offset represented either as a numeric value or a string.
pub fn deserialize_offset<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OffsetHelper {
        Num(u64),
        Str(String),
    }

    match OffsetHelper::deserialize(deserializer)? {
        OffsetHelper::Num(n) => Ok(n),
        OffsetHelper::Str(s) => s.trim().parse::<u64>().map_err(serde::de::Error::custom),
    }
}

/// Deserializes an optional byte offset represented either as a numeric value or a string.
pub fn deserialize_optional_offset<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OffsetHelper {
        Num(u64),
        Str(String),
    }

    let opt = Option::<OffsetHelper>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(OffsetHelper::Num(n)) => Ok(Some(n)),
        Some(OffsetHelper::Str(s)) => {
            let s = s.trim();
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse::<u64>().map(Some).map_err(serde::de::Error::custom)
            }
        }
    }
}

/// Status of an active or interrupted upload session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UploadSessionStatus {
    /// Upload session ID.
    pub id: String,
    /// Next expected byte offset on Meta servers.
    #[serde(deserialize_with = "deserialize_offset")]
    pub file_offset: u64,
}

/// Response returned after uploading a file chunk.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UploadChunkResponse {
    /// Uploaded file handle (`h`). Present when the upload is complete.
    #[serde(default)]
    pub h: Option<String>,

    /// Upload session ID.
    #[serde(default)]
    pub id: Option<String>,

    /// Next expected byte offset. Present when more chunks are required.
    #[serde(default, deserialize_with = "deserialize_optional_offset")]
    pub file_offset: Option<u64>,
}

impl UploadChunkResponse {
    /// Returns `true` if the upload is complete and has a file handle.
    pub fn is_complete(&self) -> bool {
        self.h.is_some()
    }

    /// Returns the file handle if the upload is complete.
    pub fn handle(&self) -> Option<&str> {
        self.h.as_deref()
    }
}
