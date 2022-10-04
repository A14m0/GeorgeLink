use serde::{Serialize, Deserialize};
use rcgen;



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



/// Generates a simple self-signed TLS certificate
pub fn _get_cert() {
    let cert = rcgen::generate_simple_self_signed(
        vec![
            "localhost".to_string(),
            "example.world".to_string()
        ]
    ).unwrap();

    println!("{}", cert.serialize_pem().unwrap());
    println!("{}", cert.serialize_private_key_pem());
}