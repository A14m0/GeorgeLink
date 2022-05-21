extern crate rustls;

mod keygen;
mod server;
mod client;
mod common;
mod log;
mod frontend;

mod cursive_gui;

use server::server_main;
use frontend::determine_gui;
use log::{LogType, log};


use clap::{Arg, App, crate_authors, crate_version};



fn main() {
    // get our CLI argument matches
    let args = App::new("GeorgeLink")
            .version(crate_version!())
			.author(crate_authors!())
			.about("Secure messaging application")
			.setting(clap::AppSettings::ArgRequiredElseHelp)
			.arg(
				Arg::new("server")
                .short('s')
                .long("server")
				.help("Run the application in a server capacity")
			)
			.arg(
				Arg::new("client")
                .short('c')
                .long("client")
				.help("Run the application in a client capacity")
			)
			.get_matches();


    // TEMPORARY 
    // handle the split between server and client
    if args.is_present("server") {
        // run the server
        server_main();//"example_keys/cert.pem", "example_keys/key.pem");//, "example_keys/root.crt");
        
    } else if args.is_present("client") {
        // run the client
        determine_gui();
    } else {
        // bail
        log(LogType::LogCrit, "[-] Please run with either `client` or `server`".to_string());
    }

}
