use std::process;
use std::sync::Arc;

use mio::net::TcpStream;

use std::convert::TryInto;
use std::fs;
use std::io;
use std::io::{BufReader, Read, Write};
use rustls::{OwnedTrustAnchor, RootCertStore};
use serde::{Serialize, Deserialize};

use crate::log::{log, LogType};
use crate::common::Message;
use crate::frontend::handle_message;

const CLIENT: mio::Token = mio::Token(0);

/// This encapsulates the TCP-level connection, some connection
/// state, and the underlying TLS-level session.
struct TlsClient {
    socket: TcpStream,
    closing: bool,
    clean_closure: bool,
    tls_conn: rustls::ClientConnection,
    inbound: Vec<Message>,
    outbound: Vec<Message>    
}

impl TlsClient {
    fn new(
        sock: TcpStream,
        server_name: rustls::ServerName,
        cfg: Arc<rustls::ClientConfig>,
    ) -> TlsClient {
        TlsClient {
            socket: sock,
            closing: false,
            clean_closure: false,
            tls_conn: rustls::ClientConnection::new(cfg, server_name).unwrap(),
            inbound: Vec::new(),
            outbound: Vec::new()
        }
    }

    /// Handles events sent to the TlsClient by mio::Poll
    fn ready(&mut self, ev: &mio::event::Event) {
        assert_eq!(ev.token(), CLIENT);

        // handle inbound messages
        while self.inbound.len() > 0 {
            handle_message(self.inbound[0].clone());
            self.inbound.remove(0);
        }

        if ev.is_readable() {
            println!("Reading data");
            self.do_read();
        }

        if ev.is_writable() {
            println!("Writing data");
            self.do_write();
        }

        if self.is_closed() {
            log(LogType::LogWarn, "Connection closed".to_string());
            process::exit(if self.clean_closure { 0 } else { 1 });
        }
    }

    fn init_connection(&mut self, user: &str) -> Result<(), io::Error> {
        let msg = Message {
                user: user.to_string(),
                mtype: crate::common::MessageType::Login,
                message: vec![0]
            };

        self.send_msg(msg)
    }

    fn send_msg(&mut self, msg: Message) -> Result<(), io::Error>{
        let s_msg = serde_json::to_string(&msg)?;
        self.tls_conn.writer().write(&s_msg.as_bytes())?;
        println!("Wrote message: {:?}", msg);
        Ok(())
    }

    /// We're ready to do a read.
    fn do_read(&mut self) {
        println!("Reading");
        // Read TLS data.  This fails if the underlying TCP connection
        // is broken.
        match self.tls_conn.read_tls(&mut self.socket) {
            Err(error) => {
                if error.kind() == io::ErrorKind::WouldBlock {
                    return;
                }
                println!("TLS read error: {:?}", error);
                self.closing = true;
                return;
            }

            // If we're ready but there's no data: EOF.
            Ok(0) => {
                println!("EOF");
                self.closing = true;
                self.clean_closure = true;
                return;
            }

            Ok(_) => {}
        };

        // Reading some TLS data might have yielded new TLS
        // messages to process.  Errors from this indicate
        // TLS protocol problems and are fatal.
        let io_state = match self.tls_conn.process_new_packets() {
            Ok(io_state) => io_state,
            Err(err) => {
                println!("TLS error: {:?}", err);
                self.closing = true;
                return;
            }
        };

        // Having read some TLS data, and processed any new messages,
        // we might have new plaintext as a result.
        //
        // Read it and then write it to stdout.
        if io_state.plaintext_bytes_to_read() > 0 {
            let mut plaintext = Vec::new();
            plaintext.resize(io_state.plaintext_bytes_to_read(), 0u8);
            self.tls_conn
                .reader()
                .read_exact(&mut plaintext)
                .unwrap();
            let msg: Message = serde_json::from_slice(&plaintext).unwrap();
            self.inbound.push(msg);
        }

        // If wethat fails, the peer might have started a clean TLS-level
        // session closure.
        if io_state.peer_has_closed() {
            self.clean_closure = true;
            self.closing = true;
        }
    }

    fn do_write(&mut self) {
        self.tls_conn
            .write_tls(&mut self.socket)
            .unwrap();
    }

    /// Registers self as a 'listener' in mio::Registry
    fn register(&mut self, registry: &mio::Registry) {
        let interest = self.event_set();
        registry
            .register(&mut self.socket, CLIENT, interest)
            .unwrap();
    }

    /// Reregisters self as a 'listener' in mio::Registry.
    fn reregister(&mut self, registry: &mio::Registry) {
        let interest = self.event_set();
        registry
            .reregister(&mut self.socket, CLIENT, interest)
            .unwrap();
    }

    /// Use wants_read/wants_write to register for different mio-level
    /// IO readiness events.
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

    fn is_closed(&self) -> bool {
        self.closing
    }
}
impl io::Write for TlsClient {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.tls_conn.writer().write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tls_conn.writer().flush()
    }
}

impl io::Read for TlsClient {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.tls_conn.reader().read(bytes)
    }
}



fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
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

/*#[cfg(feature = "dangerous_configuration")]
fn apply_dangerous_options(args: &Args, cfg: &mut rustls::ClientConfig) {
    if args.flag_insecure {
        cfg.dangerous()
            .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
    }
}*/

/// Build a `ClientConfig` from our arguments
fn make_config(ca_path: &str, certs_file: &str, key_file: &str) -> Arc<rustls::ClientConfig> {
    let mut root_store = RootCertStore::empty();
    let certfile = fs::File::open(&ca_path).expect("Cannot open CA file");
    let mut reader = BufReader::new(certfile);
    root_store.add_parsable_certificates(&rustls_pemfile::certs(&mut reader).unwrap());

    /*     root_store.add_server_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS
                .0
                .iter()
                .map(|ta| {
                    OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                }),
        );*/


    let certs = load_certs(certs_file);
    let key = load_private_key(key_file);
        

    let config = rustls::ClientConfig::builder()
        .with_cipher_suites(&[rustls::cipher_suite::TLS13_AES_256_GCM_SHA384])
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("inconsistent cipher-suite/versions selected")
        .with_root_certificates(root_store)
        .with_single_cert(certs, key)
        .expect("invalid client auth certs/key");


    Arc::new(config)
}

/// Parse some arguments, then make a TLS client connection
/// somewhere.
pub fn client_main(uname: &str, addr: std::net::SocketAddr, sname: &str, ca_path: &str, certs_file: &str, key_file: &str) {
    let config = make_config(ca_path, certs_file, key_file);

    
    log(LogType::LogInfo, "Connecting...".to_string());
    let sock = TcpStream::connect(addr).unwrap();
    let server_name = sname
        .try_into()
        .expect("invalid DNS name");
    let mut tlsclient = TlsClient::new(sock, server_name, config);

    log(LogType::LogInfo, "Connected".to_string());

    
    let mut poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(32);
    tlsclient.register(poll.registry());

    log(LogType::LogInfo, "Logging in...".to_string());

    tlsclient.init_connection(uname).unwrap();
    log(LogType::LogInfo, "Logged in".to_string());
    loop {
        poll.poll(&mut events, None).unwrap();

        for ev in events.iter() {
            tlsclient.ready(ev);
            tlsclient.reregister(poll.registry());
        }
    }
}