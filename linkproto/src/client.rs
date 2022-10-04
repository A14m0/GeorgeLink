use crate::{Message, MessageType, NoCertificateVerification};
use common::{log, LogType};

use mio::net::TcpStream;

use rustls::RootCertStore;

use std::fs::File;
use std::io::{BufReader,Read, Write};
use std::sync::Arc;


/// This encapsulates the TCP-level connection, some connection
/// state, and the underlying TLS-level session.
pub struct TlsClient {
    socket: TcpStream,
    closing: bool,
    clean_closure: bool,
    tls_conn: rustls::ClientConnection,
    inbound: Vec<Message>,
    outbound: Vec<Message> 
}

impl TlsClient {
    pub fn new(
        sock: TcpStream,
        server_name: rustls::ServerName,
        cfg: Arc<rustls::ClientConfig>,
        outbound: Vec<Message>
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
    pub fn cycle(&mut self, uname: String) {
        // initialize the connection with the server
        self.init_connection(uname).unwrap();
        

            // try read
            //println!("Reading...");
            self.do_read();

            // handle new messages
            //println!("Len: {}", self.inbound.len());
            //println!("Showing msg");
            for _m in self.inbound.iter() {
                //gui.show(m.clone());
            }
            
            //println!("Clearing");
            self.inbound.clear();

            // handle outbound message
            //println!("send outbound");
            self.send_outbound().unwrap();
            
            // try write
            //println!("writing");
            self.do_write();

            // die if we are closing
            if self.is_closed(){
                log(LogType::LogWarn, "Connection closed".to_string());
                //gui.terminate();
                //gui_handle.join().unwrap();
                std::process::exit(if self.clean_closure { 0 } else { 1 });
            }
            //println!("---------------------------------------------------");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        
    }

    /// initialize our connection with the server
    pub fn init_connection(&mut self, uname: String) -> Result<(), std::io::Error> {
        let msg = Message {
                user: uname,
                mtype: MessageType::Login,
                message: "".to_string()
            };

        self.send_msg(msg)
    }

    fn send_msg(&mut self, msg: Message) -> Result<(), std::io::Error>{
        let s_msg = serde_json::to_string(&msg)?;
        self.tls_conn.writer().write(&s_msg.as_bytes())?;
        //println!("Wrote message: {:?}", msg);
        Ok(())
    }

    pub fn send_outbound(&mut self) -> Result<(), std::io::Error> {
        for m in self.outbound.iter(){
            let s_msg = serde_json::to_string(&m)?;
            self.tls_conn.writer().write(&s_msg.as_bytes())?;
        }
        self.outbound.clear();
        Ok(())
    }

    /// We're ready to do a read.
    pub fn do_read(&mut self) {
        // Read TLS data.  This fails if the underlying TCP connection
        // is broken.
        match self.tls_conn.read_tls(&mut self.socket) {
            Err(error) => {
                if error.kind() == std::io::ErrorKind::WouldBlock {
                    return;
                }
                //println!("TLS read error: {:?}", error);
                self.closing = true;
                return;
            }

            // If we're ready but there's no data: EOF.
            Ok(0) => {
                //println!("EOF");
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
            //println!("Reading local");
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
                //println!("{:?}", m);
                let msg: Message = serde_json::from_slice(m).unwrap();
                self.inbound.push(msg);
            }
            
        } else {
            //println!("no read plaintext");
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

    pub fn do_write(&mut self) {
        self.tls_conn
            .write_tls(&mut self.socket)
            .unwrap();
    }

    pub fn is_closed(&self) -> bool {
        self.closing
    }

    pub fn get_inbound(&self) -> Vec<Message> {
        self.inbound.clone()
    }

    pub fn clear_inbound(&mut self) {
        self.inbound.clear()
    }

    pub fn add_outbound(&mut self, m: Message) {
        self.outbound.push(m);
    }
}





/////////////////////// BUILD CLIENT CONFIG /////////////////////////////////


/// Build a `ClientConfig` from our arguments
pub fn make_client_config(ca_path: &str, _certs_file: &str, _key_file: &str) -> Arc<rustls::ClientConfig> {
    let mut root_store = RootCertStore::empty();
    let certfile = File::open(&ca_path).expect("Cannot open CA file");
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




///////////////////////// IMPLEMENT TRAITS /////////////////////////////////

impl std::io::Write for TlsClient {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.tls_conn.writer().write(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.tls_conn.writer().flush()
    }
}

impl std::io::Read for TlsClient {
    fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
        self.tls_conn.reader().read(bytes)
    }
}
