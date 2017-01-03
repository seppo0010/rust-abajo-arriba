extern crate futures;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::io::Read;
use std::fs::File;

use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&mut self, request: Request) -> Self::Future {
        let mut resp = Response::new();
        match File::open(&request.path()[1..]) {
            Ok(ref mut f) => {
                let mut s = String::new();
                if let Err(_e) = f.read_to_string(&mut s) {
                    resp.status_code(500, "Internal Server Error")
                } else {
                    resp.body(&*s)
                }
            },
            Err(ref e) =>  match e.kind() {
                io::ErrorKind::NotFound => resp.status_code(404, "Not Found"),
                _ => resp.status_code(500, "Internal Server Error"),
            }
        };
        future::ok(resp)
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    TcpServer::new(Http, addr)
        .serve(|| Ok(HelloWorld));
}
