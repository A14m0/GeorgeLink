/// a console-based frontend

use crate::common::{Message, MessageType};
use crate::frontend::Gui;
use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use colored::Colorize;

/// defines the delay amount in miliseconds
const DELAY_CONST: u64 = 50;


pub struct ConsoleGUI {
    outbound: Arc<Mutex<Vec<Message>>>,
    addr: Mutex<String>,
    uname: Mutex<String>,
    is_quitting: Arc<Mutex<bool>>
}

impl Gui for ConsoleGUI {
    /// create a new frontend
    fn new(msg_vec: Arc<Mutex<Vec<Message>>>) -> Self {
        // get the server address from the user
        let mut addr = String::new();
        print!("Enter the address of the server: ");
        let _=stdout().flush();
        stdin().read_line(&mut addr).expect("Did not enter a correct string");

        // get the username to use for the interaction
        let mut uname = String::new();
        print!("Enter your handle: ");
        let _=stdout().flush();
        stdin().read_line(&mut uname).expect("Did not enter a correct string");
        
        // clean the input
        if let Some('\n')=addr.chars().next_back() {
            addr.pop();
        }
        if let Some('\r')=addr.chars().next_back() {
            addr.pop();
        }
        if let Some('\n')=uname.chars().next_back() {
            uname.pop();
        }
        if let Some('\r')=uname.chars().next_back() {
            uname.pop();
        }

        // append the port number and return
        addr.push_str(":2701");
        ConsoleGUI {
            outbound: msg_vec,
            addr: Mutex::new(addr),
            uname: Mutex::new(uname),
            is_quitting: Arc::new(Mutex::new(false))
        }
    }

    /// start the thread and return its handle
    fn start(&self) -> JoinHandle<u32> {
        let uname_loc = self.get_uname().clone();
        let outbound_loc = self.outbound.clone();
        let is_quitting_loc = self.is_quitting.clone();
            
        // start the GUI thread
        std::thread::spawn(move ||{
            // clone the required data accesses
            let iq_input = is_quitting_loc.clone();
            //let iq_output = is_quitting_loc.clone();

            // here's the main bulk of the GUI 
            let input_handle = std::thread::spawn(move ||{
                loop {
                    
                    std::thread::sleep(Duration::from_millis(10*DELAY_CONST));
                    // get the next message
                    let mut msg = String::new();
                    print!("{}{}", uname_loc.clone().blue(), "> ".green());
                    let _=stdout().flush();
                    stdin().read_line(&mut msg).expect("Did not enter a correct string");
                    if let Some('\n')=msg.chars().next_back() {msg.pop();}
                    if let Some('\r')=msg.chars().next_back() {msg.pop();}
                    

                    // push a message to send out
                    let m = Message{
                        user: uname_loc.clone(),
                        mtype: MessageType::Text,
                        message: msg.clone()
                    };

                    // save it to our vector
                    let mut o_lock = outbound_loc.lock().unwrap();
                    o_lock.push(m);

                    // check if we need to terminate ourselves
                    let is_quit_locked = iq_input.lock().unwrap();
                    if *is_quit_locked == true {
                        return 0u32;
                    }
                }
            });

            input_handle.join().unwrap();
            //output_handle.join().unwrap();

            0u32
        })
    }

    /// print a message to the console
    fn show(&self, msg: Message) {
        if msg.user != *self.uname.lock().unwrap() {
            match msg.mtype {
                MessageType::Text => println!("{} {} {}", msg.user.yellow(), " > ".green(), msg.message.green()),
                MessageType::File => println!("{} {} {}", msg.user.yellow(), " shared a file: ", msg.message.green()),
                MessageType::Login => println!("{} {}", msg.user.yellow(), " joined the server".green()),
                MessageType::Logout => println!("{} {}", msg.user.yellow(), " left the server".red())
            }
        }
    }

    /// returns the available messages for the backend to send out
    fn get_avail(&self) -> Vec<Message> {
        let o = self.outbound.lock().unwrap();
        o.clone()
    }

    /// returns the address of the server to use
    fn get_addr(&self) -> String {
        let o = self.addr.lock().unwrap();
        o.clone()
    }

    /// returns the username for the client    
    fn get_uname(&self) -> String {
        let o = self.uname.lock().unwrap();
        o.clone()
    }

    /// returns if the client is supposed to be quitting now
    fn get_disconnect(&self) -> bool {
        let o = self.is_quitting.lock().unwrap();
        o.clone()
    }

    /// terminates the GUI
    fn terminate(&mut self) {
        let mut o = self.is_quitting.lock().unwrap();
        *o = true;
    }
}