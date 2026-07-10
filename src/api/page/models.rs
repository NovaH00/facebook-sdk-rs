use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub access_token: Option<String>,
    pub category: Option<String>
}

impl Page {
    pub fn fields() -> [&'static str; 4] {
        ["id", "name", "category", "access_token"]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageScopedUser {
    pub id: String,
    pub name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_pic: Option<String>
}

impl PageScopedUser {
    pub fn fields() -> [&'static str; 5] {
        ["id", "name", "first_name", "last_name", "profile_pic"]
    }
}
