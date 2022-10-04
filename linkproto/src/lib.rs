use serde::{Serialize, Deserialize};
use rcgen;


pub mod client;
pub mod server;


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




/// Our structure for not authenticating the certificate,
/// as most of the ones we will encounter will be self-signed
/// and so by default invalid 
pub struct NoCertificateVerification {}

impl rustls::client::ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}



