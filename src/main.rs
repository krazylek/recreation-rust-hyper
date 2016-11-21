extern crate getopts;
extern crate hyper;
extern crate rustc_serialize;
extern crate url;
mod client;
mod server;

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
    let port = port_opt.as_ref().map_or(default_port, |p| { let p = p.trim(); p });

    match matches.opt_str("m") {
        Some(ref v) if v == "server" => {
            server::create(port);
        },
        Some(ref v) if v == "client" => {
            let url = &*("http://localhost:".to_string() + port);
            println!("{:?}", client::send(url));
        },
        Some(..) => println!("unknown mode"),
        None => println!("empty mode shouldn't happen"),
    };

    
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

#[test]
fn test_json_post() {
    let res = client::post_json("http://httpbin.org/post", &client::create_movie()).unwrap();
    println!("{}", res);
}
