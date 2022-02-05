/// this stores all frontend related functions and methods

use crate::common::Message;

/// Frontend interface definition
pub trait Gui {
    fn new() -> Self where Self: Sized;
    fn show(&self, msg: Message);
    fn get_avail(&self) -> Vec<Message>;
    fn get_addr(&self) -> String;
    fn get_uname(&self) -> String;
    fn get_disconnect(&self) -> bool;
}