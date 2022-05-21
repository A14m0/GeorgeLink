/// this stores all frontend related functions and methods

use crate::common::Message;
use crate::cursive_gui::CursiveGui;
use std::{sync::{Arc, Mutex}, thread::JoinHandle};

/// Frontend interface definition
pub trait Gui {
    fn new() -> Self where Self: Sized;
    fn start(&self);
}


/// launches the desired GUI frontend
pub fn determine_gui() {
    let g = CursiveGui::new();
    g.start();
}