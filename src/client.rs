use futures::{Future, Stream};
use hyper::{Client, Method};
use hyper::client::{Request};
use hyper::header::{ContentLength, ContentType};
use std::io::{self, Write};
use tokio_core::reactor::Core;
use movie::{create_movie};
use serde_json;

pub fn run(host: &str, command: &str) {
    let request = match command {
        "get" => {
            let uri = format!("http://{}/", host).parse().unwrap();
            let mut req = Request::new(Method::Get, uri);
            req
        },
        "post" => {
            let uri = format!("http://{}/", host).parse().unwrap();
            let mut req = Request::new(Method::Post, uri);
            req.set_body("Hello!");
            req
        },
        "getmovie" => {
            let uri = format!("http://{}/movie", host).parse().unwrap();
            let mut req = Request::new(Method::Get, uri);
            req
        },
        "addmovie" => {
            let uri = format!("http://{}/movie", host).parse().unwrap();
            let mut req = Request::new(Method::Post, uri);
            let movie = create_movie();
            let json = serde_json::to_string(&movie).unwrap();
            req.headers_mut().set(ContentType::json());
            req.headers_mut().set(ContentLength(json.len() as u64));
            req.set_body(json);
            req
        },
        _ => {
            println!("invalid command");
            let uri = format!("http://{}/help", host).parse().unwrap();
            let mut req = Request::new(Method::Get, uri);
            req
        },
    };

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let method = request.method().to_owned();
    let process = client.request(request).and_then(|res| {
        println!("{}: {}", method, res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        }).map(|()| { println!() })
    });

    core.run(process).unwrap();
}