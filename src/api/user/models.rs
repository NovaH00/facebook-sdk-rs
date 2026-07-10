use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,

}

impl User {
    pub fn fields() -> [&'static str; 5] {
        ["id", "name", "email", "first_name", "last_name"]
    }
}
