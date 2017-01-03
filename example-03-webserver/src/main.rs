extern crate futures;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
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
        resp.body("Hello, world!");
        future::ok(resp)
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    TcpServer::new(Http, addr)
        .serve(|| Ok(HelloWorld));
}
