use tokio_service::{Service, NewService};
use tokio_core::io::Io;
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use futures::{Future};
use std::io;
use std::net::SocketAddr;
use protocol::BaasProtocol;
use transport::BaasTransport;

struct BaasService<T> {
    inner: T,
}

struct NewBaasService<T> {
    inner: T,
}

struct BaasProto;

impl<T> Service for BaasService<T>
    where T: Service<Request = BaasProtocol, Response = String, Error = io::Error>,
          T::Future: 'static,
{
    type Request = BaasProtocol;
    type Response = String;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = io::Error>>;

    fn call(&mut self, req: BaasProtocol) -> Self::Future {
        Box::new(self.inner.call(req))
    }
}

impl<T: Io + 'static> ServerProto<T> for BaasProto {
    type Request = BaasProtocol;
    type Response = String;
    type Error = io::Error;
    type Transport = BaasTransport<T>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(BaasTransport::new(io))
    }
}

impl<T> NewService for NewBaasService<T>
    where T: NewService<Request = BaasProtocol, Response = String, Error = io::Error>,
          <T::Instance as Service>::Future: 'static
{
    type Request = BaasProtocol;
    type Response = String;
    type Error = io::Error;
    type Instance = BaasService<T::Instance>;

    fn new_service(&self) -> Result<Self::Instance, Self::Error> {
        let inner = try!(self.inner.new_service());
        Ok(BaasService { inner: inner })
    }
}

pub fn serve<T>(addr: SocketAddr, new_service: T)
    where T: NewService<Request = BaasProtocol, Response = String, Error = io::Error> + Send + Sync + 'static,
{
    let new_service = NewBaasService { inner: new_service };

    TcpServer::new(BaasProto, addr)
        .serve(new_service);
}
