
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub message: Option<String>,
    pub story: Option<String>,
    pub created_time: Option<DateTime<Utc>>,
    pub updated_time: Option<DateTime<Utc>>,
    pub permalink_url: Option<String>,
}

impl Post {
    pub fn fields() -> [&'static str; 6] {
        ["id", "message", "story", "created_time", "updated_time", "permalink_url"]
    }
}
