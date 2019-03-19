use command::{Response, Command};
use codec::PixelflutClientCodec;
use error::Error;

use futures::{Async, Future, Poll, Sink, StartSend, Stream};

use tokio_core::reactor::Handle;
use tokio_core::net::{TcpStream, TcpStreamNew};

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;

use std::net::SocketAddr;

pub struct Client {
    host: SocketAddr,
}

impl Client {
    pub fn new<H: Into<SocketAddr>>(host: H) -> Client {
        Client { host: host.into() }
    }

    pub fn connect(&self, handle: &Handle) -> ClientConnectFuture {
        let tcp_stream = TcpStream::connect(&self.host, handle);
        ClientConnectFuture { inner: tcp_stream }
    }
}

pub struct ClientConnectFuture {
    inner: TcpStreamNew,
}

impl Future for ClientConnectFuture {
    type Item = PixelflutClientTransport<TcpStream>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let framed = try_ready!(self.inner.poll()).framed(PixelflutClientCodec);
        let pixelflut_transport = PixelflutClientTransport::new(framed);

        Ok(Async::Ready(pixelflut_transport))
    }
}

pub struct PixelflutClientTransport<T>
where
T: AsyncRead + AsyncWrite
{
    inner: Framed<T, PixelflutClientCodec>,
}

impl<T> PixelflutClientTransport<T>
where
T: AsyncRead + AsyncWrite,
{
    fn new(inner: Framed<T, PixelflutClientCodec>) -> PixelflutClientTransport<T> {
        PixelflutClientTransport { inner }
    }
}

impl<T> Stream for PixelflutClientTransport<T>
where
T: AsyncRead + AsyncWrite,
{
    type Item = Response;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let command = try_ready!(self.inner.poll());
        if let Some(command) = command {
            Ok(Async::Ready(Some(command)))
        } else {
            Ok(Async::NotReady)
        }
    }
}

impl<T> Sink for PixelflutClientTransport<T>
where
T: AsyncRead + AsyncWrite,
{
    type SinkItem = Command;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Ok(self.inner.start_send(item)?)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        match self.inner.poll_complete()? {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(()) => Ok(Async::Ready(()))
        }
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.inner.close()
    }
}
