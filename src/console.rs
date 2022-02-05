/// a console-based frontend

use crate::common::{Message, MessageType};
use crate::frontend::Gui;
use std::io::{stdin, stdout, Write};
use colored::Colorize;



pub struct ConsoleGUI {
    outbound: Vec<Message>,
    addr: String,
    uname: String,
    is_quitting: bool
}

impl Gui for ConsoleGUI {
    /// create a new frontend
    fn new() -> Self {
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
            outbound: Vec::new(),
            addr,
            uname,
            is_quitting: false
        }
    }

    /// print a message to the console
    fn show(&self, msg: Message) {
        match msg.mtype {
            MessageType::Text => println!("{} {} {}", msg.user.yellow(), " > ".green(), msg.message.green()),
            MessageType::File => println!("{} {} {}", msg.user.yellow(), " shared a file: ", msg.message.green()),
            MessageType::Login => println!("{} {}", msg.user.yellow(), " joined the server".green()),
            MessageType::Logout => println!("{} {}", msg.user.yellow(), " left the server".red())
        }
    }

    /// returns the available messages for the backend to send out
    fn get_avail(&self) -> Vec<Message> {
        self.outbound.clone()
    }

    /// returns the address of the server to use
    fn get_addr(&self) -> String {
        self.addr.clone()
    }

    /// returns the username for the client    
    fn get_uname(&self) -> String {
        self.uname.clone()
    }

    /// returns if the client is supposed to be quitting now
    fn get_disconnect(&self) -> bool {
        self.is_quitting
    }
}