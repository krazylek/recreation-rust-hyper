extern crate getopts;
extern crate futures;
extern crate hyper;
extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate url;

mod server;
mod client;
mod movie;

use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("p", "port", "set server port", "PORT");
    opts.optopt("s", "host", "set server host", "HOST");
    opts.optopt("m", "mode", "set program mode: client | server", "MODE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", &args[0]);
        print!("{}", opts.usage(&brief));
        return;
    }

    let port = matches.opt_str("p").unwrap_or("8090".to_owned());
    let hostname = matches.opt_str("s").unwrap_or("127.0.0.1".to_owned());
    let host = format!("{}:{}", hostname, port);

    match matches.opt_str("m") {
        Some(ref v) if v == "server" => {
            server::create(&host);
        },
        Some(ref v) if v == "client" => {
            client::run(&host, "get");
            client::run(&host, "post");
            client::run(&host, "getmovie");
            client::run(&host, "addmovie");
        },
        Some(..) => println!("unknown mode"),
        None => println!("empty mode shouldn't happen"),
    };
}