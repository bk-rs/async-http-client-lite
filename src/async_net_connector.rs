use std::io;

use async_net::{AsyncToSocketAddrs, TcpStream};
use async_trait::async_trait;

use crate::connector::*;

pub struct AsyncNetTcpConnector;

#[async_trait]
impl<A> Connector<TcpStream, A> for AsyncNetTcpConnector
where
    A: AsyncToSocketAddrs + Send + Sync + 'static,
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

    use std::path::Path;

    use async_net::unix::UnixStream;

    pub struct AsyncNetUnixConnector;

    #[async_trait]
    impl<A> Connector<UnixStream, A> for AsyncNetUnixConnector
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
