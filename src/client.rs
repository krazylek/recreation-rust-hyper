extern crate hyper;
extern crate rustc_serialize;

use hyper::{Client};
use std::io::Read;
use url::form_urlencoded;
use rustc_serialize::{Encodable, json};
use server::{Movie};

pub type Query<'a> = Vec<(&'a str, &'a str)>;

fn post(url: &str, body:&str) -> hyper::Result<String> {
    println!("post request to {}", url);
    let client = Client::new();
    let mut response = try!(client.post(url).body(&body[..]).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

pub fn post_json<T>(url: &str, payload: &T) -> hyper::Result<String>
    where T: Encodable {
    println!("creating json post request");
    let body = json::encode(payload).unwrap();
    post(&url, &body)
}

pub fn post_data(url: &str, query:Query) -> hyper::Result<String> {
    let body: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(query.into_iter())
        .finish();
    post(&url, &body)
}

pub fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    println!("buf: {}", &buf);
    Ok(buf)
}

pub fn send(url: &str) -> Vec<String> {
    vec![ 
        get_content(url).unwrap(),
        post_data(url, vec![("keyA", "valueB"), ("foo", "bar")]).unwrap(),
        post_json(url, &create_movie()).unwrap()
    ]
}

pub fn create_movie() -> Movie {
    Movie {
        title: "You Only Live Twice".to_owned(),
        year: 1967,
        bad_guy: "Blofeld".to_owned(),
    }
}
