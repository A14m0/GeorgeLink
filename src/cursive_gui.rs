/////// ALL CODE SO FAR EXAMPLE CODE FROM 
/// https://github.com/gyscos/cursive/tree/main/cursive/examples
/////////////////////////////////////////
/// 
use crate::client::build_client;
use crate::client::TlsClient;
use crate::common::{Message, MessageType};
use crate::frontend::Gui;
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::ListView;
use cursive::views::{Panel, EditView, OnEventView, LinearLayout, TextView};
use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::theme::Effect;
use cursive::theme::Style;
use std::sync::mpsc::{channel, Sender};
use cursive::utils::markup::StyledString;

pub struct CursiveGui {
    uname: String,
}

impl Gui for CursiveGui {
    /// create a new instance of the Curses GUI
    fn new() -> Self {
        CursiveGui { uname: "test".to_string() }
    }

    /// run the GUI
    fn start(&self) {
        // create initial cursive context and set theme
        let mut siv = cursive::default();
        siv.load_theme_file("/home/morpheus/src/repos/BingoChado/GeorgeLink/src/assets/style.toml").unwrap();

        // create our global callback for when we send a message
        siv.add_global_callback(Event::Key(Key::Enter), |s| {
            println!("Caught ctrl-enter");
            s.quit()
        });

        let (remote_sender, remote_receiver) = channel();
        let (mut local_sender, local_receiver) = channel();
        let thread_uname = self.uname.clone();

        let handle = std::thread::spawn(move || {
            let mut tls_client = build_client("localhost", "example_keys/ecdsa/ca.cert", "example_keys/ecdsa/client.req", "example_keys/ecdsa/client.key", "127.0.0.1:2701");

            match tls_client.init_connection(thread_uname) {
                Ok(_) => (),
                Err(e) => panic!("Failed to connect: {}", e)
            };

            // loop until we get a message (or have one to send)
            loop {
                // see if we are trying to send anything
                match local_receiver.try_recv() {
                    Ok(a) => {
                        // we have a message, send it
                        tls_client.add_outbound(a);
                    },
                    Err(_) => 
                        ()
                };

                // see if we are trying to receive anything
                tls_client.do_read();
                let inbound = tls_client.get_inbound();

                // handle new messages
                for m in inbound.iter() {
                    loop {
                        match remote_sender.send(m.clone()) {
                            Ok(_) => break,
                            Err(_) => ()
                        }
                    }
                }
            
                // clear the messages and write them to the server
                tls_client.clear_inbound();
                tls_client.send_outbound().unwrap();
                tls_client.do_write();
                if tls_client.is_closed(){
                    return;
                }
            }
        });
    
        // set up the layout
        siv.add_fullscreen_layer(
            LinearLayout::vertical()
                .child(
                    Panel::new(
                        ListView::new()
                    )
                    .title(self.uname.clone())
                    .wrap_with(OnEventView::new))
                .child(
                    Panel::new(
                        EditView::new().fixed_height(4)
                    )
            ).full_screen()
        );

        // run the GUI
        siv.set_window_title("GeorgeLink");
        siv.runner().refresh();
        let mut message_count = 0;
        loop {
            siv.runner().step();
            if !siv.is_running() {
                break;
            }
    
            let mut needs_refresh = false;
            //Non blocking channel receiver.
            for m in remote_receiver.try_iter() {
                siv.runner().call_on_name("messages", |messages: &mut LinearLayout| {
                    needs_refresh = true;
                    message_count += 1;
                    match self.gen_new_msg(m) {
                        Ok(a) => {
                            messages.add_child(TextView::new(a));
                            
                        },
                        Err(_) => message_count -= 1
                    }
                    if message_count <= 14 {
                        messages.remove_child(0);
                    }
                });
            }
            if needs_refresh {
                siv.runner().refresh();
            }
        }
    }
}

impl CursiveGui {
    /// Shows `msg` in the GUI
    fn gen_new_msg(&self, msg: Message) -> Result<StyledString, ()> {
        let mut styled_message = StyledString::default();
        match msg.mtype {
            MessageType::Login => {
                styled_message.append(StyledString::styled("[", Color::Dark(BaseColor::Green)));
                styled_message.append(StyledString::styled(msg.user, Color::Dark(BaseColor::Yellow)));
                styled_message.append(StyledString::styled("]", Color::Dark(BaseColor::Green)));
                styled_message.append(StyledString::styled(" logged in", Color::Light(BaseColor::Green)));
            },
            MessageType::Logout => {
                styled_message.append(StyledString::styled("[", Color::Dark(BaseColor::Green)));
                styled_message.append(StyledString::styled(msg.user, Color::Dark(BaseColor::Yellow)));
                styled_message.append(StyledString::styled("]", Color::Dark(BaseColor::Green)));
                styled_message.append(StyledString::styled(" logged off", Color::Dark(BaseColor::Red)));
            },
            MessageType::Text => {
                styled_message.append(StyledString::styled("[", Color::Dark(BaseColor::Green)));
                styled_message.append(StyledString::styled(msg.user, Color::Dark(BaseColor::Yellow)));
                styled_message.append(StyledString::styled("]", Color::Dark(BaseColor::Green)));
                styled_message.append(StyledString::styled(" > ", Color::Dark(BaseColor::Red)));
                styled_message.append(StyledString::plain(msg.message));
            },
            MessageType::File => {
                styled_message.append(StyledString::styled("[", Color::Dark(BaseColor::Green)));
                styled_message.append(StyledString::styled(msg.user, Color::Dark(BaseColor::Yellow)));
                styled_message.append(StyledString::styled("]", Color::Dark(BaseColor::Green)));
                styled_message.append(StyledString::styled(" sent a file", Color::Dark(BaseColor::White)));
            },
            MessageType::RespOK => {return Err(())}
        }

        Ok(styled_message)

    }
}

