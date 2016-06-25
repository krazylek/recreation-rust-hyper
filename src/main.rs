extern crate getopts;
extern crate hyper;
extern crate num;
extern crate rustc_serialize;
extern crate url;
mod client;
mod server;

use server::{Movie};
use getopts::Options;
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("p", "port", "set server port", "PORT");
    opts.reqopt("m", "mode", "set program mode: client | server", "MODE");
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

    let default_port = "8090";
    let port_opt = matches.opt_str("p");
    let port = port_opt.as_ref().map_or(default_port, |ref p| { let p = p.trim(); p });

    match matches.opt_str("m") {
        Some(ref v) if v == "server" => {
            server::create(port);
        },
        Some(ref v) if v == "client" => {
            let url = &*("http://localhost:".to_string() + port);
            println!("{:?}", client::post_json(url, &create_movie()).unwrap());
        },
        Some(..) => println!("unknown mode"),
        None => println!("empty mode shouldn't happen"),
    };

    
}

fn create_movie() -> Movie {
    Movie {
        title: "You Only Live Twice".to_owned(),
        year: 1967,
        bad_guy: "Blofeld".to_owned(),
    }
}


#[test]
fn test_simple_get() {
    let res = client::get_content("http://www.visual-salade.com").unwrap();
    println!("{}", res);
}

#[test]
fn test_simple_post() {
    let query = vec![("keyA", "valueB"), ("foo", "bar")];
    let res = client::post_data("http://httpbin.org/post", query).unwrap();
    println!("{}", res);
}
