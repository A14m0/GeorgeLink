use mio::net::{TcpStream,TcpListener};
use rustls::{RootCertStore};
use rustls::server::AllowAnyAnonymousOrAuthenticatedClient;

use std::sync::Arc;
use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::fs::File;


use common::{log,LogType};


use crate::{Message, MessageType};

/// This binds together a TCP listening socket, some outstanding
/// connections, and a TLS server configuration.
pub struct TlsServer {
    server: TcpListener,
    connections: HashMap<mio::Token, OpenConnection>,
    next_id: usize,
    tls_config: Arc<rustls::ServerConfig>,
}

impl TlsServer {
    pub fn new(server: TcpListener, cfg: Arc<rustls::ServerConfig>) -> Self {
        TlsServer {
            server,
            connections: HashMap::new(),
            next_id: 2,
            tls_config: cfg,
        }
    }

    pub fn accept(&mut self, registry: &mio::Registry) -> Result<(), std::io::Error> {
        loop {
            match self.server.accept() {
                Ok((socket, addr)) => {
                    log(LogType::LogInfo, format!("Accepting new connection from {:?}", addr));

                    let tls_conn =
                        rustls::ServerConnection::new(Arc::clone(&self.tls_config)).unwrap();
                    
                    let token = mio::Token(self.next_id);
                    self.next_id += 1;

                    let mut connection = OpenConnection::new(socket, token, tls_conn);
                    connection.register(registry);
                    self.connections
                        .insert(token, connection);
                    log(LogType::LogInfo, "Successfully connected".to_string());
                }
                Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
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

    pub fn conn_event(&mut self, registry: &mio::Registry, event: &mio::event::Event) {
        let token = event.token();

        if self.connections.contains_key(&token) {
            let msgs = self.connections
                .get_mut(&token)
                .unwrap()
                .ready(registry, event);

            /*
            TODO: 
            --------------------------------------------------------------------
            Seems to be some confusion in the channel with overlapping messages
            Could we flush/clear socket or smth?

            Example of an offending line and corresponding log

            [123, 34, 117, 115, 101, 114, 34, 58, 34, 34, 44, 34, 109, 116, 121, 112, 101, 34, 58, 34, 82, 101, 115, 112, 79, 75, 34, 44, 34, 109, 101, 115, 115, 97, 103, 101, 34, 58, 34, 34, 125, 123, 34, 117, 115, 101, 114, 34, 58, 34, 97, 97, 97, 97, 34, 44, 34, 109, 116, 121, 112, 101, 34, 58, 34, 84, 101, 120, 116, 34, 44, 34, 109, 101, 115, 115, 97, 103, 101, 34, 58, 34, 97, 34, 125]
            [ERR]: Failed to serialize message: trailing characters at line 1 column 42
            [WARN]: Skipping...

            Could it also be in part due to a sync issue with the server?
            Could we accidentally be flagging Mio poll events with our writes?

            */


            // broadcast all messages received
            for msg in msgs.iter() {
                self.broadcast_message(msg.clone());
            }

            if self.connections[&token].is_closed() {
                self.connections.remove(&token);
            }
        }
    }

    /// Broadcasts a message to all associated clients
    pub fn broadcast_message(&mut self, msg: Message) {
        println!("Broadcasting message to {} clients", self.connections.len());
        // filter out response messages
        if msg.mtype != MessageType::RespOK {
            for (_,t) in self.connections.iter_mut() {
                t.send_msg(msg.clone()).unwrap();
            }
        }
    }
}

/// This is a connection which has been accepted by the server,
/// and is currently being served.
///
/// It has a TCP-level stream, a TLS-level connection state, and some
/// other state/metadata.
struct OpenConnection {
    socket: TcpStream,
    token: mio::Token,
    closing: bool,
    closed: bool,
    tls_conn: rustls::ServerConnection,
    queue: Vec<Message>
}

impl OpenConnection {
    fn new(
        socket: TcpStream,
        token: mio::Token,
        tls_conn: rustls::ServerConnection,
    ) -> OpenConnection {
        OpenConnection {
            socket,
            token,
            closing: false,
            closed: false,
            tls_conn,
            queue: Vec::new()
        }
    }

    /// We're a connection, and we have something to do.
    fn ready(&mut self, registry: &mio::Registry, ev: &mio::event::Event) -> Vec<Message>{
        // If we're readable: read some TLS.  Then
        // see if that yielded new plaintext.  Then
        // see if the backend is readable too.
        let mut ret = Vec::new();
        if ev.is_readable() {
            self.do_tls_read();
            self.try_plain_read();    
        }

        ret.append(&mut self.queue);

        if ev.is_writable() {
            self.do_tls_write_and_handle_error();
        }

        if self.closing {
            let _ = self
                .socket
                .shutdown(std::net::Shutdown::Both);
            self.closed = true;
            self.deregister(registry);
        } else {
            self.reregister(registry);
        }

        ret
    }

    /// send a Message structure to the client
    fn send_msg(&mut self, msg: Message) -> Result<(), std::io::Error>{
        let s_msg = serde_json::to_string(&msg)?;
        println!("{}", s_msg);
        self.tls_conn.writer().write(&s_msg.as_bytes())?;
        println!("Wrote to client");
        self.do_tls_read();
        Ok(())
    }
    
    /// Reads data from the TLS socket 
    fn do_tls_read(&mut self) {
        // Read some TLS data.
        match self.tls_conn.read_tls(&mut self.socket) {
            Err(err) => {
                if let std::io::ErrorKind::WouldBlock = err.kind() {
                    return;
                }

                log(LogType::LogErr, format!("Read error {:?}", err));
                self.closing = true;
                return;
            }
            Ok(0) => {
                log(LogType::LogWarn, "Client disconnect".to_string());
                self.closing = true;
                return;
            }
            Ok(_) => {}
        };

        // Process newly-received TLS messages.
        if let Err(err) = self.tls_conn.process_new_packets() {
            log(LogType::LogErr, format!("cannot process packet: {:?}", err));

            // last gasp write to send any alerts
            self.do_tls_write_and_handle_error();

            self.closing = true;
        }
    }

    /// Attempts to read the plaintext
    fn try_plain_read(&mut self){
        // Read and process all available plaintext.
        if let Ok(io_state) = self.tls_conn.process_new_packets() {
            if io_state.plaintext_bytes_to_read() > 0 {
                let mut buf = Vec::new();
                buf.resize(io_state.plaintext_bytes_to_read(), 0u8);

                self.tls_conn
                    .reader()
                    .read_exact(&mut buf)
                    .unwrap();

                log(LogType::LogInfo, format!("plaintext read {:?}", buf.len()));
                self.incoming_plaintext(&buf);
                println!("{:?}", buf);

                // deserialize the message
                match serde_json::from_slice(&buf.clone()) {
                    Ok(a) => self.queue.push(a),
                    Err(e) => {
                        log(LogType::LogErr, format!("Failed to serialize message: {}", e));
                        log(LogType::LogWarn, "Skipping...".to_string());
                    }
                }
            }
        }
    }

    /// Process some amount of received plaintext.
    fn incoming_plaintext(&mut self, buf: &[u8]) {        
        self.tls_conn
            .writer()
            .write_all(buf)
            .unwrap();
    }

    /// Raw write to the TLS socket
    fn tls_write(&mut self) -> std::io::Result<usize> {
        self.tls_conn
            .write_tls(&mut self.socket)
    }

    /// Writes the data to the socket, terminating if it encounters an error
    fn do_tls_write_and_handle_error(&mut self) {
        let rc = self.tls_write();
        if rc.is_err() {
            log(LogType::LogErr, format!("write failed {:?}", rc));
            self.closing = true;
        }
    }

    /// Registers a token for Mio events
    fn register(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry
            .register(&mut self.socket, self.token, event_set)
            .unwrap();
    }

    /// Reregisters a token for a Mio event
    fn reregister(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry
            .reregister(&mut self.socket, self.token, event_set)
            .unwrap();
    }

    /// Deregisters a Mio token 
    fn deregister(&mut self, registry: &mio::Registry) {
        registry
            .deregister(&mut self.socket)
            .unwrap();
    }

    /// What IO events we're currently waiting for,
    /// based on wants_read/wants_write.
    fn event_set(&self) -> mio::Interest {
        let rd = self.tls_conn.wants_read();
        let wr = self.tls_conn.wants_write();

        if rd && wr {
            mio::Interest::READABLE | mio::Interest::WRITABLE
        } else if wr {
            mio::Interest::WRITABLE
        } else {
            mio::Interest::READABLE
        }
    }

    /// Says whether or not the connection is closed
    fn is_closed(&self) -> bool {
        self.closed
    }
}


/// Loads a certificate from a file
fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}

/// Loads a private key from a file
fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = File::open(filename).expect("cannot open private key file");
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


/// Builds a TlsServer configuration structure
pub fn make_server_config(cert_path: &str, key_path: &str) -> Arc<rustls::ServerConfig> {
    let roots = load_certs(cert_path);
    let mut client_auth_roots = RootCertStore::empty();
    for root in roots {
        client_auth_roots.add(&root).unwrap();
    }
    
    let client_auth = AllowAnyAnonymousOrAuthenticatedClient::new(client_auth_roots);
                                            // AllowAnyAnonymousOrAuthenticatedClient::new(client_auth_roots)
    
    

    let certs = load_certs(
        cert_path,
    );
    log(LogType::LogInfo, "Certificates loaded".to_string());
    
    let privkey = load_private_key(
        key_path,
    );
    log(LogType::LogInfo, "RSA key loaded".to_string());
    
    
    let mut config = rustls::ServerConfig::builder()
        .with_cipher_suites(&[rustls::cipher_suite::TLS13_AES_256_GCM_SHA384])
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("inconsistent cipher-suites/versions specified")
        .with_client_cert_verifier(client_auth)
        .with_single_cert(certs, privkey)
        .expect("bad certificates/private key");

    config.key_log = Arc::new(rustls::KeyLogFile::new());

    log(LogType::LogInfo, "Configuration complete".to_string());
    Arc::new(config)
}
