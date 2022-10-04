use std::sync::Arc;

use mio::net::{TcpListener, TcpStream};

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::{BufReader, Read, Write};
use std::net;




use linkproto::server::{TlsServer, make_server_config};




fn main() {
    let addr: net::SocketAddr = "127.0.0.1:2701".parse().unwrap();
    
    let config = make_server_config("example_keys/ecdsa/ca.cert", "example_keys/ecdsa/ca.key");

    let mut listener = TcpListener::bind(addr).expect("cannot listen on port");
    // TODO: Move this into the TlsServer stuff
    let mut poll = mio::Poll::new().unwrap();
    poll.registry()
        .register(&mut listener, LISTENER, mio::Interest::READABLE)
        .unwrap();

    

    let mut tlsserv = TlsServer::new(listener, config);

    let mut events = mio::Events::with_capacity(256);
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                LISTENER => {
                    tlsserv
                        .accept(poll.registry())
                        .expect("error accepting socket");
                }
                _ => tlsserv.conn_event(poll.registry(), event),
            }
        }
    }
}



























































/*

//#[cfg(features="server")]
//mod rustls;
use std::sync::Arc;
use std::io;
use std::io::{BufReader, Read, Write};
use std::net::{self, TcpListener, TcpStream};
use crate::keygen;
use crate::log::{LogType, log};

use mio::Token;
use rustls::ServerConnection;
use rustls::server::{
    AllowAnyAnonymousOrAuthenticatedClient, AllowAnyAuthenticatedClient, NoClientAuth,
};


const LISTENER: mio::Token = mio::Token(0);


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
    log(LogType::LogInfo, "Certificates loaded".to_string());
    let privkey = load_private_key(
        key_file,
    );
    log(LogType::LogInfo, "Keys loaded".to_string());
    
   
    // create the config structure
    let config = rustls::ServerConfig::builder()
        .with_cipher_suites(&[rustls::cipher_suite::TLS13_AES_256_GCM_SHA384])
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("inconsistent cipher-suites/versions specified")
        .with_client_cert_verifier(client_auth)
        .with_single_cert(certs, privkey)
        .expect("bad certificates/private key");

    log(LogType::LogInfo, "Configuration complete".to_string());
    
        
    Arc::new(config)
}


/// our server's view of a client
struct Client {
    socket: TcpStream,
    token: mio::Token,
    connection: rustls::ServerConnection,
    terminate: bool
}

impl Client {
    fn new(socket: TcpStream, token: Token, connection: rustls::ServerConnection) -> Self {
        Client {
            socket,
            token,
            connection,
            terminate: false
        }
    }

    fn read_tls(&mut self) {
        match self.connection.read_tls(&mut self.socket) {
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock { return } 
                
                log(LogType::LogErr, format!("Failed to read TLS content: {:?}", e));
                self.terminate= true;
                return;
            },
            Ok(0) => {
                return;
            },
            Ok(_) => {}
        }

        // handle errors
        if let Err(e) = self.connection.process_new_packets() {
            log(LogType::LogErr, format!("Failed to process packet: {:?}", e));

            // last gasp write to send any alerts
            self.do_tls_write_and_handle_error();

            self.terminate = true;
        }
    }

    fn tls_write(&mut self) -> io::Result<usize> {
        self.connection
            .write_tls(&mut self.socket)
    }

    fn do_tls_write_and_handle_error(&mut self) {
        let rc = self.tls_write();
        if rc.is_err() {
            log(LogType::LogErr, format!("Failed to write TLS data: {:?}", rc));
            self.terminate = true;
        }
    }
}


/// Our Server structure, holds all the things the server needs to
/// operate 
pub struct Server {
    server: TcpListener,
    clients: std::collections::HashMap<mio::Token, Client>,
    next_id: usize,
    config: Arc<rustls::ServerConfig>,
    poll: mio::Poll,
}

impl Server {
    /// creates a new default instance, listening on port 2701
    pub fn new(certs_file: &str, key_file: &str) -> Self{
        let config = make_config(certs_file, key_file);
        Server::new_custom("0.0.0.0:2701", config)
    }

    /// creates an instance with custom bind port and address 
    pub fn new_custom(addr_str: &str, config: Arc<rustls::ServerConfig>) -> Self {
        let addr: std::net::SocketAddr = addr_str.parse().unwrap();
        let mut l = TcpListener::bind(addr_str).unwrap();
        let mut poll = mio::Poll::new().unwrap();
        poll.registry()
            .register(&mut l, LISTENER, mio::Interest::READABLE);
        Server {
            server: l,
            clients: std::collections::HashMap::new(),
            next_id: 2,
            config,
            poll
        }
    }

    /// accept an incoming connection
    fn accept(&mut self) -> Result<(), io::Error> {
    
        keygen::get_cert();
        log(LogType::LogInfo, "Server running".to_string());
        loop {
            match self.server.accept() {
                Ok((socket, addr)) => {
                    log(LogType::LogInfo, format!("Accepting new connection from {:?}", addr));

                    let tls_conn =
                        rustls::ServerConnection::new(Arc::clone(&self.config)).unwrap();
                    
                    let token = mio::Token(self.next_id);
                    self.next_id += 1;

                    let mut connection = Client::new(socket, token, tls_conn);
                    connection.register(self.poll.registry());
                    self.clients
                        .insert(token, connection);
                }
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(err) => {
                    println!(
                        "encountered error while accepting connection; err={:?}",
                        err
                    );
                    return Err(err);
                }
            }
        }
    }

    fn conn_event(&mut self, event: &mio::event::Event) {
        let token = event.token();

        if self.clients.contains_key(&token) {
            self.clients
                .get_mut(&token)
                .unwrap()
                .ready(self.poll.registry(), event);

            if self.clients[&token].is_closed() {
                self.clients.remove(&token);
            }
        }
    }

    pub fn run(&mut self) {
        log(LogType::LogInfo, "Server run".to_string());

        
        self.poll.registry()
            .register(&mut self.server, LISTENER, mio::Interest::READABLE)
            .unwrap();

        let mut events = mio::Events::with_capacity(256);
    
        loop {
            self.poll.poll(&mut events, None).unwrap();
    
            for event in events.iter() {
                match event.token() {
                    LISTENER => {
                        self
                            .accept()
                            .expect("error accepting socket");
                    }
                    _ => self.conn_event(event),
                }
            }
        }
    } 

}*/



