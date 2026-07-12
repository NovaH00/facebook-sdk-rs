use serde::{Serialize, Deserialize, Serializer, Deserializer};

/// A Facebook profile picture returned by the Graph API.
///
/// The API returns pictures as `{ data: { url, height, width, is_silhouette } }`.
/// This struct flattens that nesting so fields are accessed directly
/// (e.g. `picture.url` instead of `picture.data.url`).
#[derive(Debug, Clone)]
pub struct Picture {
    /// The profile picture URL.
    pub url: String,
    /// The picture height in pixels.
    pub height: Option<i32>,
    /// The picture width in pixels.
    pub width: Option<i32>,
    /// Whether this is a silhouette (default profile photo).
    pub is_silhouette: Option<bool>,
}

impl<'de> Deserialize<'de> for Picture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawData {
            url: String,
            height: Option<i32>,
            width: Option<i32>,
            is_silhouette: Option<bool>,
        }
        #[derive(Deserialize)]
        struct Raw {
            data: RawData,
        }
        let raw = Raw::deserialize(deserializer)?;
        Ok(Picture {
            url: raw.data.url,
            height: raw.data.height,
            width: raw.data.width,
            is_silhouette: raw.data.is_silhouette,
        })
    }
}

impl Serialize for Picture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct RawData<'a> {
            url: &'a str,
            height: Option<i32>,
            width: Option<i32>,
            is_silhouette: Option<bool>,
        }
        #[derive(Serialize)]
        struct Raw<'a> {
            data: RawData<'a>,
        }
        Raw {
            data: RawData {
                url: &self.url,
                height: self.height,
                width: self.width,
                is_silhouette: self.is_silhouette,
            },
        }
        .serialize(serializer)
    }
}

/// A Facebook Page managed by the authenticated user.
///
/// Returned by the `/me/accounts` endpoint. Contains the Page's
/// access token which can be used to create Page-scoped API clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// The Page's Facebook ID.
    pub id: String,
    /// The Page's display name.
    pub name: String,
    /// The Page's access token (long-lived, for Page-scoped API calls).
    pub access_token: Option<String>,
    /// The Page's category (e.g. "App Page").
    pub category: Option<String>,
    /// The Page's profile picture.
    pub picture: Option<Picture>,
}

impl Page {
    /// Returns the field names available on this type for Graph API field selection.
    pub fn fields() -> [&'static str; 5] {
        ["id", "name", "access_token", "category", "picture"]
    }
}

/// A user within a Page's scope.
///
/// Returned when looking up a user by ID with a Page access token.
/// The ID is Page-scoped (PSID) — it is unique to that Page, not global.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageScopedUser {
    /// The Page-scoped user ID (PSID).
    pub id: String,
    /// The user's display name (visible to the Page).
    pub name: String,
}

impl PageScopedUser {
    /// Returns the field names available on this type for Graph API field selection.
    pub fn fields() -> [&'static str; 2] {
        ["id", "name"]
    }
}
