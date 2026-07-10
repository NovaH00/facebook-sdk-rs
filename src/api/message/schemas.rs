
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMessageResponse {
    pub message_id: String,
    pub recipient_id: String
}
