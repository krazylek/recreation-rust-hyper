extern crate hyper;
use rustc_serialize::{json};
use hyper::server::{Server, Request, Response};
use hyper::status::{StatusCode};
use std::io::Write; // for res.write_all
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Movie {
    pub title: String,
    pub year: usize,
    pub bad_guy: String
}

fn handle_request(mut req: Request, mut res: Response) {
    println!("processing req");
    match req.method {
        hyper::Post => {
            println!("POST");
            let mut payload = String::new();
            req.read_to_string(&mut payload).unwrap();
            let movie: Movie = json::decode(&*payload).unwrap();
            let body = movie.title.as_bytes();
            let mut res = res.start().unwrap();
            res.write_all(body).unwrap();
        },
        hyper::Get => {
            println!("GET");
            res.send(b"Hello World!\n").unwrap();
        },
        _ => {
            println!("OTHER");
            *res.status_mut() = StatusCode::MethodNotAllowed;
        }
    }
}

pub fn create(port: &str) {
    println!("server started on port {}", port);
    let addr = "0.0.0.0:".to_string() + port;
    Server::http(&*addr).unwrap().handle(handle_request).unwrap();
}
