mod server;
mod client;
use server::server::server_main;
use client::client::client_main;

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
        server_main();
    } else if args.is_present("client") {
        // run the client
        client_main();
    } else {
        // bail
        println!("[-] Please run with either `client` or `server`");
    }

}
