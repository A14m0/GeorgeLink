use serde::{Serialize, Deserialize};



/// defines our Message type
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType{
    Login,
    Logout,
    RespOK,
    Text,
    File
}

/// Holds messages sent over the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub user: String,
    pub mtype: MessageType,
    pub message: String
}

impl From<&Message> for std::string::String {
    fn from(a: &Message) -> Self { serde_json::to_string(a).unwrap() }
}
