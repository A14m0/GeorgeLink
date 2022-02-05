/// this stores all frontend related functions and methods

use crate::common::Message;
use std::{sync::{Arc, Mutex}, thread::JoinHandle};

/// Frontend interface definition
pub trait Gui {
    fn new(msg_vec: Arc<Mutex<Vec<Message>>>) -> Self where Self: Sized;
    fn start(&self) -> JoinHandle<u32>;
    fn show(&self, msg: Message);
    fn get_avail(&self) -> Vec<Message>;
    fn get_addr(&self) -> String;
    fn get_uname(&self) -> String;
    fn get_disconnect(&self) -> bool;
    fn terminate(&mut self);
}