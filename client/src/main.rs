use std::convert::TryInto;
use common::{log, LogType};
use linkproto::Message;
use linkproto::client::make_client_config;
use linkproto::client::TlsClient;


/// Parse some arguments, then make a TLS client connection
/// somewhere.
pub fn build_client(sname: &str, ca_path: &str, certs_file: &str, key_file: &str, addr: &str) -> TlsClient{
    // generate a configuration
    let config = make_client_config(ca_path, certs_file, key_file);

    // connect to the remote server
    log(LogType::LogInfo, "Connecting...".to_string());
    let shared_message: Vec<Message> = Vec::new();

    // TODO: Move this into TcpClient creation
    let sock = TcpStream::connect(addr.parse().unwrap()).unwrap();
    let server_name = sname
        .try_into()
        .expect("invalid DNS name");

    // set up the tls client structure
    TlsClient::new(sock, server_name, config, shared_message)
}