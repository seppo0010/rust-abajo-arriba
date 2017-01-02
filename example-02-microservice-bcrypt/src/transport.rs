use std::{mem,io};
use futures::{Async,Poll, Stream};
use tokio_core::io::Io;
use nom;
use ::protocol::BaasProtocol;

pub struct LowLevelBaasTransport<T> {
	inner: T,
	read_buffer: Vec<u8>,
	write_buffer: io::Cursor<Vec<u8>>,
}

impl<T> LowLevelBaasTransport<T> {
	pub fn new(inner: T) -> LowLevelBaasTransport<T> {
		LowLevelBaasTransport {
			inner: inner,
			read_buffer: vec![],
			write_buffer: io::Cursor::new(vec![]),
		}
	}
}

pub enum Error {
	IOError(io::Error),
	NomError(nom::ErrorKind),
}

impl<T> Stream for LowLevelBaasTransport<T>
	where T: Io
{
	type Item = BaasProtocol;
	type Error = Error;

	fn poll(&mut self) -> Poll<Option<BaasProtocol>, Error> {
		loop {
			let parsed = match BaasProtocol::parse(&*self.read_buffer) {
				nom::IResult::Done(read, res) => Some((read.len(), res)),
				nom::IResult::Incomplete(_) => None,
				nom::IResult::Error(e) => return Err(Error::NomError(e)),
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
					return Err(Error::IOError(e));
				}
			}
		}
	}
}
