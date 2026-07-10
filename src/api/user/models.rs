use serde::{Serialize, Deserialize};

/// A Facebook user profile returned by the `/me` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// The user's Facebook ID.
    pub id: String,
    /// The user's display name.
    pub name: String,
    /// The user's email address (requires `email` permission).
    pub email: Option<String>,
}

impl User {
    /// Returns the field names available on this type for Graph API field selection.
    pub fn fields() -> [&'static str; 3] {
        ["id", "name", "email"]
    }
}
