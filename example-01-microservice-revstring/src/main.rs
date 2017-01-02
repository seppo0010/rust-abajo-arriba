extern crate tokio_line as line;
extern crate service_fn;

fn main() {
	let addr = "127.0.0.1:12345".parse().unwrap();
	line::service::serve(
		addr,
		|| {
			Ok(service_fn::service_fn(|msg: String| {
				Ok(msg.chars().rev().collect::<String>())
			}))
	});
}
