use serde::{Serialize, Deserialize};

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
    /// The Page's profile picture URL.
    pub picture: Option<String>,
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
