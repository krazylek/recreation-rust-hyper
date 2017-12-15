extern crate futures;
extern crate hyper;

use futures::{future, Future, Stream};
use hyper::server::{Http, Request, Response, Service};
use hyper::{Chunk, Method, StatusCode};
use hyper::header::{ContentType};
use movie::{create_movie, Movie};
use serde_json;
use std::io;



const PHRASE: &'static str = "Hello, World!";

// fn to_uppercase(chunk: Chunk) -> Chunk {
//     let uppered = chunk.iter()
//         .map(|byte| byte.to_ascii_uppercase())
//         .collect::<Vec<u8>>();
//     Chunk::from(uppered)
// }

fn reverse(chunk: Chunk) -> Response {
    let reversed = chunk.iter()
        .rev()
        .cloned()
        .collect::<Vec<u8>>();
    Response::new()
        .with_body(reversed)
}

#[derive(Serialize, Debug)]
struct PostResult {
    result: String,
    inserted: bool
}

struct MovieBase;

impl Service for MovieBase {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;


    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                println!("GET /");
                Box::new(future::ok(Response::new().with_body(PHRASE)))
            },
            (&Method::Post, "/") => {
                println!("POST /");
                Box::new(
                    req.body()
                        .concat2()
                        .map(reverse)
                )
            },
            (&Method::Post, "/movie") => {
                println!("POST /movie");
                let res = req.body().concat2().map(|body: Chunk| {

                    let new_movie: Movie = serde_json::from_slice(&body)
                        .map_err(|e| {
                            io::Error::new(io::ErrorKind::Other, e)
                        })
                        .unwrap();

                    // save in db here...
                    
                    let result = PostResult { 
                        result: format!("Received \"{}\"", new_movie.title.to_uppercase()),
                        inserted: false
                    };

                    let serialized = serde_json::to_string(&result).unwrap();

                    Response::new()
                        .with_header(ContentType::json())
                        .with_body(serialized)
                });
                Box::new(res)
            },
            (&Method::Get, "/movie") => {
                println!("GET /movie");
                let movie = create_movie();
                let serialized = serde_json::to_string(&movie).unwrap();
                let res = Response::new()
                    .with_header(ContentType::json())
                    .with_body(serialized);
                Box::new(future::ok(res))
            },
            _ => {
                println!("NO ROUTE");
                Box::new(future::ok(
                    Response::new().with_status(StatusCode::NotFound)
                ))
            }
        }
    }
}

pub fn create(host: &str) {
    println!("server started on {}", &host);
    let addr = host.parse().unwrap();
    let server = Http::new().bind(&addr, ||Ok(MovieBase)).unwrap();
    server.run().unwrap()
}
