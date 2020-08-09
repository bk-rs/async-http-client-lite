use std::io;

use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

pub(crate) use crate::client::{Client, ClientInnerStream};
pub(crate) use crate::client_proxy::ClientProxy;
pub(crate) use crate::client_tls::ClientTls;

#[async_trait]
pub trait Connector<S, A>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    async fn connect(
        addr: A,
        client_proxy: Option<ClientProxy>,
        client_tls: Option<ClientTls>,
    ) -> io::Result<Client<ClientInnerStream<S>>>;
}
