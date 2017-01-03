extern crate futures;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::str;
use std::io;
use std::io::{Read, Write};
use std::fs::{create_dir_all, File};
use std::path::Path;

use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

struct HelloWorld;

impl HelloWorld {
    fn read(&mut self, request: Request) -> Result<String, (u32, &str)> {
        File::open(&request.path()[1..]).and_then(|ref mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s).map(|_| s)
        }).map_err(|e| {
            match e.kind() {
                io::ErrorKind::NotFound => (404, "Not Found"),
                _ => (500, "Internal Server Error"),
            }
        })
    }

    fn write(&mut self, request: Request) -> Result<(), (u32, &str)> {
        let p = request.path();
        let path = Path::new(&p[1..]);
        if let Some(parent) = path.parent() {
            if parent.is_file() {
                return Err((400, "Parent path is a file"))
            }
            if ! parent.is_dir() {
                if let Err(_) = create_dir_all(parent) {
                    return Err((500, "Failed to create directory"))
                }
            }
        }
        File::create(path).and_then(|mut f| {
            f.write(request.body().as_slice()).map(|_| ())
        }).map_err(|_| (500, "Internal Server Error"))
    }
}

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&mut self, request: Request) -> Self::Future {
        let mut resp = Response::new();
        let r = match request.method() {
            "POST" => self.write(request).map(|_| "OK".to_owned()),
            _ => self.read(request),
        };
        match r {
            Ok(e) => { resp.body(&*e); },
            Err((status, message)) => { resp.status_code(status, message); },
        }
        future::ok(resp)
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    TcpServer::new(Http, addr)
        .serve(|| Ok(HelloWorld));
}
