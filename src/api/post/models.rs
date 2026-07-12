use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// A Facebook Page post.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Post {
    /// The post ID.
    pub id: String,
    /// The text content of the post.
    pub message: Option<String>,
    /// The story text (if the post was created from a story).
    pub story: Option<String>,
    /// When the post was created.
    pub created_time: Option<DateTime<Utc>>,
    /// When the post was last updated.
    pub updated_time: Option<DateTime<Utc>>,
    /// Permanent URL to the post.
    pub permalink_url: Option<String>,
}

impl Post {
    /// Returns the field names available on this type for Graph API field selection.
    pub fn fields() -> [&'static str; 6] {
        ["id", "message", "story", "created_time", "updated_time", "permalink_url"]
    }
}
