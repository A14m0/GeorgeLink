/////// ALL CODE SO FAR EXAMPLE CODE FROM 
/// https://github.com/gyscos/cursive/tree/main/cursive/examples
/////////////////////////////////////////
/// 
use crate::client::build_client;
use crate::client::TlsClient;
use crate::frontend::Gui;
use cursive::event::{Event, Key, EventTrigger};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, OnEventView, TextArea, LinearLayout, TextView};

pub struct CursiveGui {
    tls_client: TlsClient,
}

impl Gui for CursiveGui {
    fn new() -> Self {
        let tls_client = build_client("localhost", "example_keys/ecdsa/ca.cert", "example_keys/ecdsa/client.req", "example_keys/ecdsa/client.key", "127.0.0.1:2701");

        CursiveGui { tls_client }
    }

    fn start(&self) {
        //
        let mut siv = cursive::default();
    
        siv.add_layer(
            LinearLayout::vertical()
                .child(TextView::new("Messages go here").scrollable().min_height(5))
                .child(cursive::views::OnEventView::new(
                    EditView::new().scrollable().max_height(4)
                )
                .on_event(Event::Ctrl(Key::Enter), |s| {
                    println!("Caught ctrl-enter");
                    s.quit()
                }))
        );

        siv.set_window_title("GeorgeLink");
    
        siv.run();
    }
}

