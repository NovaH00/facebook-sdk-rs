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

/// The response returned by Facebook when a post is created successfully.
///
/// Returned by [`PostApi::create_post`](crate::api::post::PostApi::create_post).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePostResponse {
    /// The ID of the newly created post.
    pub id: String,
}

/// A media item to attach to a post.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum PostMedia {
    /// An image with a public URL and an optional caption.
    Photo {
        /// Public URL of the image.
        url: String,
        /// Optional caption specifically for this image.
        caption: Option<String>,
    },
    /// A video with a public URL and an optional caption.
    Video {
        /// Public URL of the video file.
        url: String,
        /// Optional caption specifically for this video.
        caption: Option<String>,
    },
}

impl PostMedia {
    /// Creates a photo media item without a caption.
    pub fn photo(url: impl Into<String>) -> Self {
        Self::Photo {
            url: url.into(),
            caption: None,
        }
    }

    /// Creates a photo media item with a custom caption.
    pub fn photo_with_caption(url: impl Into<String>, caption: impl Into<String>) -> Self {
        Self::Photo {
            url: url.into(),
            caption: Some(caption.into()),
        }
    }

    /// Creates a video media item without a caption.
    pub fn video(url: impl Into<String>) -> Self {
        Self::Video {
            url: url.into(),
            caption: None,
        }
    }

    /// Creates a video media item with a custom caption.
    pub fn video_with_caption(url: impl Into<String>, caption: impl Into<String>) -> Self {
        Self::Video {
            url: url.into(),
            caption: Some(caption.into()),
        }
    }
}

impl From<String> for PostMedia {
    fn from(url: String) -> Self {
        Self::photo(url)
    }
}

impl From<&str> for PostMedia {
    fn from(url: &str) -> Self {
        Self::photo(url)
    }
}

