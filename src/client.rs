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
use crate::frontend::Gui;
use crate::console::ConsoleGUI;
        

const CLIENT: mio::Token = mio::Token(0);



/// default GUI interface for when no GUI feature is present
/// note that this is never used, and is simply here to make
/// the rust compiler happy about structure sizes :)
struct DefaultGUI {}
impl Gui for DefaultGUI {
    fn new() -> Self {DefaultGUI{}}
    fn show(&self, msg: Message){}
    fn get_avail(&self) -> Vec<Message>{Vec::new()}
    fn get_addr(&self) -> String {"".to_string()}
    fn get_uname(&self) -> String {"".to_string()}
    fn get_disconnect(&self) -> bool {true}
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

/// This encapsulates the TCP-level connection, some connection
/// state, and the underlying TLS-level session.
struct TlsClient<'a> {
    socket: TcpStream,
    closing: bool,
    clean_closure: bool,
    tls_conn: rustls::ClientConnection,
    inbound: Vec<Message>,
    outbound: Vec<Message>   ,
    gui: &'a dyn Gui 
}

impl TlsClient<'_> {
    fn new(
        sock: TcpStream,
        server_name: rustls::ServerName,
        cfg: Arc<rustls::ClientConfig>,
        gui: &dyn Gui
    ) -> TlsClient {
        TlsClient {
            socket: sock,
            closing: false,
            clean_closure: false,
            tls_conn: rustls::ClientConnection::new(cfg, server_name).unwrap(),
            inbound: Vec::new(),
            outbound: Vec::new(),
            gui
        }
    }

    /// Handles events sent to the TlsClient by mio::Poll
    fn ready(&mut self, ev: &mio::event::Event) {
        assert_eq!(ev.token(), CLIENT);

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

    fn init_connection(&mut self) -> Result<(), io::Error> {
        let msg = Message {
                user: self.gui.get_uname(),
                mtype: crate::common::MessageType::Login,
                message: "".to_string()
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

            // parse the messages
            let plen = plaintext.len();
            let mut msg_vec: Vec<&[u8]> = Vec::new();
            let mut last: usize = 0;
            for i in 0..plen {
                if plaintext[i] == b'}' {
                    msg_vec.push(&plaintext[last..i+1]);
                    last = i + 1;
                }
            }
            // send them to the gui
            for m in msg_vec {
                println!("{:?}", m);
                let msg: Message = serde_json::from_slice(m).unwrap();
                self.inbound.push(msg);
            }
            
        }

        // If we have received messages, make the backend handle it
        if self.inbound.len() > 0 {
            for msg in self.inbound.iter(){
                self.gui.show(msg.clone());
            }
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
impl io::Write for TlsClient<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.tls_conn.writer().write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tls_conn.writer().flush()
    }
}

impl io::Read for TlsClient<'_> {
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
        

    let mut config = rustls::ClientConfig::builder()
        .with_cipher_suites(&[rustls::cipher_suite::TLS13_AES_256_GCM_SHA384])
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("inconsistent cipher-suite/versions selected")
        .with_root_certificates(root_store)
        .with_no_client_auth();

    config.dangerous().set_certificate_verifier(Arc::new(NoCertificateVerification {}));



    Arc::new(config)
}

/// Parse some arguments, then make a TLS client connection
/// somewhere.
pub fn client_main(sname: &str, ca_path: &str, certs_file: &str, key_file: &str) {
    // generate a configuration
    let config = make_config(ca_path, certs_file, key_file);

    // connect to the remote server
    log(LogType::LogInfo, "Connecting...".to_string());
    let gui = ConsoleGUI::new();
    let addr: std::net::SocketAddr = gui.get_addr().parse().unwrap(); 
    let sock = TcpStream::connect(addr).unwrap();
    let server_name = sname
        .try_into()
        .expect("invalid DNS name");

    // set up the tls client structure
    let mut tlsclient = TlsClient::new(sock, server_name, config, &gui);
    log(LogType::LogInfo, "Connected".to_string());

    // set up polling
    let mut poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(32);
    tlsclient.register(poll.registry());

    // log in to the remote
    log(LogType::LogInfo, "Logging in...".to_string());
    tlsclient.init_connection().unwrap();
    log(LogType::LogInfo, "Logged in".to_string());

    // event loop
    loop {
        poll.poll(&mut events, None).unwrap();

        for ev in events.iter() {
            tlsclient.ready(ev);
            tlsclient.reregister(poll.registry());
        }
    }
}