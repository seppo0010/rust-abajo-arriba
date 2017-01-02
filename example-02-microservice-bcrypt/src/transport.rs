use std::{mem,io};
use futures::{Async, AsyncSink, Poll, Stream, Sink, StartSend};
use tokio_core::io::Io;
use nom;
use ::protocol::BaasProtocol;

pub struct BaasTransport<T> {
	inner: T,
	read_buffer: Vec<u8>,
	write_buffer: io::Cursor<Vec<u8>>,
}

impl<T> BaasTransport<T> {
	pub fn new(inner: T) -> BaasTransport<T> {
		BaasTransport {
			inner: inner,
			read_buffer: vec![],
			write_buffer: io::Cursor::new(vec![]),
		}
	}
}

impl<T> Stream for BaasTransport<T>
	where T: Io
{
	type Item = BaasProtocol;
	type Error = io::Error;

	fn poll(&mut self) -> Poll<Option<BaasProtocol>, io::Error> {
		loop {
			let parsed = match BaasProtocol::parse(&*self.read_buffer) {
				nom::IResult::Done(read, res) => Some((self.read_buffer.len() - read.len() - 1, res)),
				nom::IResult::Incomplete(_) => None,
				nom::IResult::Error(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
			};
			if let Some((n, res)) = parsed {
				let tail = self.read_buffer.split_off(n+1);
				let mut line = mem::replace(&mut self.read_buffer, tail);
				line.truncate(n);
				return Ok(Async::Ready(Some(res)));
			}
			match self.inner.read_to_end(&mut self.read_buffer) {
				Ok(0) => return Ok(Async::Ready(None)),
				Ok(_) => {},
				Err(e) => {
					if e.kind() == io::ErrorKind::WouldBlock {
						return Ok(Async::NotReady);
					}
					return Err(e);
				}
			}
		}
	}
}

impl<T> Sink for BaasTransport<T>
	where T: Io
{
	type SinkItem = String;
	type SinkError = io::Error;

	fn start_send(&mut self, req: String) -> StartSend<String, io::Error> {
		if self.write_buffer.position() < self.write_buffer.get_ref().len() as u64 {
			self.write_buffer.get_mut().append(&mut req.into_bytes());
			self.write_buffer.get_mut().push(b'\n');
			return Ok(AsyncSink::Ready);
		}
		let mut bytes = req.into_bytes();
		bytes.push(b'\n');
		self.write_buffer = io::Cursor::new(bytes);
		Ok(AsyncSink::Ready)
	}

	fn poll_complete(&mut self) -> Poll<(), io::Error> {
		loop {
			let res = {
				let buf = {
					let pos = self.write_buffer.position() as usize;
					let buf = &self.write_buffer.get_ref()[pos..];
					if buf.is_empty() {
						return Ok(Async::Ready(()));
					}
					buf
				};
				self.inner.write(buf)
			};
			match res {
				Ok(mut n) => {
					n += self.write_buffer.position() as usize;
					self.write_buffer.set_position(n as u64)
				}
				Err(e) => {
					if e.kind() == io::ErrorKind::WouldBlock {
						return Ok(Async::NotReady);
					}
					return Err(e)
				}
			}
		}
	}
}
