extern crate rustls;

mod keygen;
mod server;
mod client;
mod common;
mod log;
mod frontend;
mod console;

use server::server_main;
use client::client_main;
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
        
        client_main( "localhost", "example_keys/ecdsa/ca.cert", "example_keys/ecdsa/client.req", "example_keys/ecdsa/client.key");
    } else {
        // bail
        println!("[-] Please run with either `client` or `server`");
    }

}
