use serde::{Serialize, Deserialize};



/// defines our Message type
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MessageType{
    Login,
    Text,
    File
}

/// Holds messages sent over the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub user: String,
    pub mtype: MessageType,
    pub message: Vec<u8>
}
