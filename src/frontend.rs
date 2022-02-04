/// this stores all frontend related functions and methods

use crate::common::{Message, MessageType};

pub fn handle_message(msg: Message) {
    println!("{:?}", msg);
}