#[macro_use] extern crate nom;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate futures;
extern crate service_fn;
extern crate bcrypt;

use std::sync::{Arc, Mutex};

mod protocol;
mod service;
mod transport;

fn main() {
	let addr = "127.0.0.1:12345".parse().unwrap();
	let size = Arc::new(Mutex::new(13));
	service::serve(
		addr,
		move || {
			let size = size.clone();
			Ok(service_fn::service_fn(move |msg: protocol::BaasProtocol| {
				let s = match msg {
					protocol::BaasProtocol::SetCost(s) => {
						let mut data = size.lock().unwrap();
						*data = s;
						"OK".to_owned()
					},
					protocol::BaasProtocol::Hash(s) => {
						bcrypt::hash(&*s, *size.lock().unwrap()).unwrap()
					},
					protocol::BaasProtocol::Verify(s, h) => {
						match bcrypt::verify(&*s, &*h).unwrap() {
							true => "valid",
							false => "invalid"
						}.to_owned()
					},
				};
				Ok(s)
			}))
	});
}
