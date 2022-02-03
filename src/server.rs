

//#[cfg(features="server")]
//mod rustls;
use std::sync::Arc;
use std::io;
use std::io::{BufReader, Read, Write};
use std::net::{self, TcpListener, TcpStream};
use crate::log::{LogType, log};

use rustls::ServerConnection;
use rustls::server::{
    AllowAnyAnonymousOrAuthenticatedClient, AllowAnyAuthenticatedClient, NoClientAuth,
};



// could be handy
// https://www.linode.com/docs/guides/create-a-self-signed-tls-certificate/



/// originally from:
/// https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsserver.rs
fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = std::fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}


/// originally from:
/// https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsserver.rs
fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = std::fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}

/// originally from: 
/// https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsserver.rs
fn try_read(r: io::Result<usize>) -> io::Result<Option<usize>> {
    match r {
        Ok(len) => Ok(Some(len)),
        Err(e) => {
            if e.kind() == io::ErrorKind::WouldBlock {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

/// modified function originally from: 
/// https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsserver.rs
fn make_config(certs_file: &str, key_file: &str) -> Arc<rustls::ServerConfig> {
    let roots = load_certs(certs_file);
    let mut client_auth_roots = rustls::RootCertStore::empty();
    for root in roots {
        client_auth_roots.add(&root).unwrap();
    }
        
    // declare we want only authenticated clients
    let client_auth = AllowAnyAuthenticatedClient::new(client_auth_roots);

    // load certificates and keys
    let certs = load_certs(
        certs_file,
    );
    let privkey = load_private_key(
        key_file,
    );
   
    // create the config structure
    let config = rustls::ServerConfig::builder()
        .with_cipher_suites(&[rustls::cipher_suite::TLS13_AES_256_GCM_SHA384])
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("inconsistent cipher-suites/versions specified")
        .with_client_cert_verifier(client_auth)
        .with_single_cert(certs, privkey)
        .expect("bad certificates/private key");
        
    Arc::new(config)
}


/// our server's view of a client
struct Client {
    socket: TcpStream,
    connection: rustls::ServerConnection
}

impl Client {
    fn new(socket: TcpStream, connection: rustls::ServerConnection) -> Self {
        Client {
            socket,
            connection
        }
    }
}


/// Our Server structure, holds all the things the server needs to
/// operate 
pub struct Server {
    server: TcpListener,
    clients: Vec<ServerConnection>,
    config: Arc<rustls::ServerConfig>
}

impl Server {
    /// creates a new default instance, listening on port 2701
    fn new(key_file: &str, certs_file: &str) -> Self{
        let config = make_config(key_file, certs_file);
        Server::new_custom("0.0.0.0:2701", config)
    }

    /// creates an instance with custom bind port and address 
    fn new_custom(addr_str: &str, config: Arc<rustls::ServerConfig>) -> Self {
        let l = TcpListener::bind(addr_str).unwrap();
        Server {
            server: l,
            clients: Vec::new(),
            config
        }
    }

    /// accept an incoming connection
    fn accept(&mut self) -> Result<(), io::Error> {
        loop {
            match self.server.accept() {
                Ok((socket, addr)) => {
                    log(LogType::LogInfo, format!("Accepting connection from {:?}", addr));
                    
                    let conn = rustls::ServerConnection::new(Arc::clone(&self.config)).unwrap();

                    let mut connection = OpenConnection::new(socket, conn);
                    self.clients.push(connection);
                
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(e) => {
                    log(LogType::LogCrit, format!("Failed to accept connection: {:?}", e));
                    return Err(e);
                }
            }
        }
    }

}



