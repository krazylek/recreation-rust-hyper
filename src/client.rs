extern crate hyper;
extern crate rustc_serialize;

use hyper::{Client};
use std::io::Read;
use url::form_urlencoded;
use rustc_serialize::{Encodable, json};

pub type Query<'a> = Vec<(&'a str, &'a str)>;

fn post(url: &str, body:&str) -> hyper::Result<String> {
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
