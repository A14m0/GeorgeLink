extern crate rustls;

mod keygen;
mod server;
mod client;
mod log;
use server::Server;
use client::client::client_main;
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
        //server_main();
        let mut s = Server::new("example_keys/cert.pem", "example_keys/key.pem");//, "example_keys/root.crt");
        s.accept().unwrap();
        s.run();
    } else if args.is_present("client") {
        // run the client
        client_main();
    } else {
        // bail
        println!("[-] Please run with either `client` or `server`");
    }

}
