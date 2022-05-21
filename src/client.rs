use std::process;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use mio::net::TcpStream;

use std::convert::TryInto;
use std::fs;
use std::io;
use std::io::{BufReader, Read, Write};
use rustls::{RootCertStore};

use crate::log::{log, LogType};
use crate::common::{Message, MessageType};
use crate::frontend::Gui;



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
pub struct TlsClient {
    socket: TcpStream,
    closing: bool,
    clean_closure: bool,
    tls_conn: rustls::ClientConnection,
    inbound: Vec<Message>,
    outbound: Arc<Mutex<Vec<Message>>> 
}

impl TlsClient {
    fn new(
        sock: TcpStream,
        server_name: rustls::ServerName,
        cfg: Arc<rustls::ClientConfig>,
        outbound: Arc<Mutex<Vec<Message>>>
    ) -> TlsClient {
        TlsClient {
            socket: sock,
            closing: false,
            clean_closure: false,
            tls_conn: rustls::ClientConnection::new(cfg, server_name).unwrap(),
            inbound: Vec::new(),
            outbound
        }
    }

    /// primary logic loop
    fn cycle(&mut self, uname: String) {
        // initialize the connection with the server
        self.init_connection(uname).unwrap();
        

            // try read
            println!("Reading...");
            self.do_read();

            // handle new messages
            //println!("Len: {}", self.inbound.len());
            println!("Showing msg");
            for m in self.inbound.iter() {
                //gui.show(m.clone());
            }
            
            println!("Clearing");
            self.inbound.clear();

            // handle outbound message
            println!("send outbound");
            self.send_outbound().unwrap();
            
            // try write
            println!("writing");
            self.do_write();

            // die if we are closing
            if self.is_closed(){
                log(LogType::LogWarn, "Connection closed".to_string());
                //gui.terminate();
                //gui_handle.join().unwrap();
                process::exit(if self.clean_closure { 0 } else { 1 });
            }
            println!("---------------------------------------------------");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        
    }

    /// initialize our connection with the server
    fn init_connection(&mut self, uname: String) -> Result<(), io::Error> {
        let msg = Message {
                user: uname,
                mtype: crate::common::MessageType::Login,
                message: "".to_string()
            };

        self.send_msg(msg)
    }

    fn send_msg(&mut self, msg: Message) -> Result<(), io::Error>{
        let s_msg = serde_json::to_string(&msg)?;
        self.tls_conn.writer().write(&s_msg.as_bytes())?;
        //println!("Wrote message: {:?}", msg);
        Ok(())
    }

    fn send_outbound(&mut self) -> Result<(), io::Error> {
        let mut o_lock = self.outbound.lock().unwrap();
        for m in o_lock.iter(){
            let s_msg = serde_json::to_string(&m)?;
            self.tls_conn.writer().write(&s_msg.as_bytes())?;
        }
        o_lock.clear();
        Ok(())
    }

    /// We're ready to do a read.
    fn do_read(&mut self) {
        // Read TLS data.  This fails if the underlying TCP connection
        // is broken.
        match self.tls_conn.read_tls(&mut self.socket) {
            Err(error) => {
                if error.kind() == io::ErrorKind::WouldBlock {
                    println!("Blocking");
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
                log(LogType::LogErr, format!("TLS error: {:?}", err));
                self.closing = true;
                return;
            }
        };

        // Having read some TLS data, and processed any new messages,
        // we might have new plaintext as a result.
        //
        // Read it and then write it to stdout.
        if io_state.plaintext_bytes_to_read() > 0 {
            println!("Reading local");
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
            
        } else {
            println!("no read plaintext");
        }

        // If wethat fails, the peer might have started a clean TLS-level
        // session closure.
        if io_state.peer_has_closed() {
            self.clean_closure = true;
            self.closing = true;
        } else {
            self.send_msg(Message{
                user: "".to_string(),
                mtype: MessageType::RespOK,
                message: "".to_string()

            }).unwrap();
        }
    }

    fn do_write(&mut self) {
        self.tls_conn
            .write_tls(&mut self.socket)
            .unwrap();
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



/*fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
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
}*/

/*#[cfg(feature = "dangerous_configuration")]
fn apply_dangerous_options(args: &Args, cfg: &mut rustls::ClientConfig) {
    if args.flag_insecure {
        cfg.dangerous()
            .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
    }
}*/

/// Build a `ClientConfig` from our arguments
fn make_config(ca_path: &str, _certs_file: &str, _key_file: &str) -> Arc<rustls::ClientConfig> {
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


    //let certs = load_certs(certs_file);
    //let key = load_private_key(key_file);
        

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
pub fn build_client(sname: &str, ca_path: &str, certs_file: &str, key_file: &str, addr: &str) -> TlsClient{
    // generate a configuration
    let config = make_config(ca_path, certs_file, key_file);

    // connect to the remote server
    log(LogType::LogInfo, "Connecting...".to_string());
    let shared_message: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(Vec::new()));
    let sock = TcpStream::connect(addr.parse().unwrap()).unwrap();
    let server_name = sname
        .try_into()
        .expect("invalid DNS name");

    // set up the tls client structure
    TlsClient::new(sock, server_name, config, shared_message.clone())
}