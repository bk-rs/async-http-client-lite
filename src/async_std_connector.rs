use std::io;

use async_std::net::{TcpStream, ToSocketAddrs};
use async_trait::async_trait;

use crate::connector::*;

pub struct AsyncStdTcpConnector;

#[async_trait]
impl<A> Connector<TcpStream, A> for AsyncStdTcpConnector
where
    A: ToSocketAddrs + Send + Sync + 'static,
    A::Iter: Send,
{
    async fn connect(
        addr: A,
        client_proxy: Option<ClientProxy>,
        client_tls: Option<ClientTls>,
    ) -> io::Result<Client<ClientInnerStream<TcpStream>>> {
        let stream = TcpStream::connect(addr).await?;

        Client::with(stream, client_proxy, client_tls).await
    }
}

//
//
//
#[cfg(unix)]
mod unix_ {
    use super::*;

    use async_std::os::unix::net::UnixStream;
    use async_std::path::Path;

    pub struct AsyncStdUnixConnector;

    #[async_trait]
    impl<A> Connector<UnixStream, A> for AsyncStdUnixConnector
    where
        A: AsRef<Path> + Send + 'static,
    {
        async fn connect(
            addr: A,
            client_proxy: Option<ClientProxy>,
            client_tls: Option<ClientTls>,
        ) -> io::Result<Client<ClientInnerStream<UnixStream>>> {
            let stream = UnixStream::connect(addr).await?;

            Client::with(stream, client_proxy, client_tls).await
        }
    }
}
#[cfg(unix)]
pub use unix_::*;
