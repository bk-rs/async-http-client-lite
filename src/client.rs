use std::io;

use async_stream_packed::HttpClientInnerStream;
use futures_io::{AsyncRead, AsyncWrite};

//
#[cfg(feature = "http_tunnel")]
use async_stream_packed::HttpClientProxy;

//
use crate::client_proxy::ClientProxy;
use crate::client_tls::ClientTls;

#[cfg(feature = "http_tunnel")]
use crate::client_http_tunnel::UnionableHttpTunnelClientGrader;

#[cfg(feature = "tls")]
use crate::client_tls::UnionableTlsClientUpgrader;

//
//
//
pub struct Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    inner_stream: S,
}

impl<S> Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    pub fn new(inner_stream: S) -> Self {
        Self { inner_stream }
    }

    pub fn get_ref(&self) -> &S {
        &self.inner_stream
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.inner_stream
    }

    pub fn into_inner(self) -> S {
        self.inner_stream
    }
}

//
//
//
cfg_if::cfg_if! {
    if #[cfg(all(not(feature = "tls"), not(feature = "http_tunnel")))] {
        pub(crate) type ClientInnerStream<S> = HttpClientInnerStream<S, (), (), ()>;
    } else if #[cfg(all(feature = "tls", not(feature = "http_tunnel")))] {
        pub(crate) type ClientInnerStream<S> = HttpClientInnerStream<S, (), (), UnionableTlsClientUpgrader>;
    } else if #[cfg(all(not(feature = "tls"), feature = "http_tunnel"))] {
        pub(crate) type ClientInnerStream<S> = HttpClientInnerStream<S, (), UnionableHttpTunnelClientGrader, ()>;
    } else if #[cfg(all(feature = "tls", feature = "http_tunnel"))] {
        pub(crate) type ClientInnerStream<S> = HttpClientInnerStream<S, UnionableTlsClientUpgrader, UnionableHttpTunnelClientGrader, UnionableTlsClientUpgrader>;
    } else {
    }
}

impl<S> Client<ClientInnerStream<S>>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    cfg_if::cfg_if! {
        if #[cfg(all(not(feature = "tls"), not(feature = "http_tunnel")))] {
            pub async fn with(stream: S, client_proxy: Option<ClientProxy>, client_tls: Option<ClientTls>) -> io::Result<Self> {
                if client_proxy.is_some() {
                    debug_assert!(false, "unreachable")
                }
                if client_tls.is_some() {
                    debug_assert!(false, "unreachable")
                }

                let inner_stream = HttpClientInnerStream::new(stream, None, None).await?;
                Ok(Self::new(inner_stream))
            }
        } else if #[cfg(all(feature = "tls", not(feature = "http_tunnel")))] {
            pub async fn with(stream: S, client_proxy: Option<ClientProxy>, client_tls: Option<ClientTls>) -> io::Result<Self> {
                if client_proxy.is_some() {
                    debug_assert!(false, "unreachable")
                }

                let tls_upgrader = client_tls.map(|x| x.into_tls_upgrader());
                let inner_stream = HttpClientInnerStream::new(stream, None, tls_upgrader).await?;
                Ok(Self::new(inner_stream))
            }
        } else if #[cfg(all(not(feature = "tls"), feature = "http_tunnel"))] {
            pub async fn with(stream: S, client_proxy: Option<ClientProxy>, client_tls: Option<ClientTls>) -> io::Result<Self> {
                if client_tls.is_some() {
                    debug_assert!(false, "unreachable")
                }

                let proxy = if let Some(client_proxy) = client_proxy {
                    match client_proxy {
                        ClientProxy::Http(client_http_tunnel) => Some(HttpClientProxy::http(
                            client_http_tunnel.into_http_tunnel_grader(),
                        )),
                        ClientProxy::Https(_, _) => unreachable!(),
                    }
                } else {
                    None
                };

                let inner_stream = HttpClientInnerStream::new(stream, proxy, None).await?;
                Ok(Self::new(inner_stream))
            }
        } else if #[cfg(all(feature = "tls", feature = "http_tunnel"))] {
            pub async fn with(stream: S, client_proxy: Option<ClientProxy>, client_tls: Option<ClientTls>) -> io::Result<Self> {
                let proxy = if let Some(client_proxy) = client_proxy {
                    match client_proxy {
                        ClientProxy::Http(client_http_tunnel) => Some(HttpClientProxy::http(
                            client_http_tunnel.into_http_tunnel_grader(),
                        )),
                        ClientProxy::Https(proxy_client_tls, client_http_tunnel) => Some(HttpClientProxy::https(
                            proxy_client_tls.into_tls_upgrader(),
                            client_http_tunnel.into_http_tunnel_grader(),
                        )),
                    }
                } else {
                    None
                };

                let tls_upgrader = client_tls.map(|x| x.into_tls_upgrader());

                let inner_stream = HttpClientInnerStream::new(stream, proxy, tls_upgrader).await?;
                Ok(Self::new(inner_stream))
            }
        } else {
            compile_error("unreachable")
        }
    }
}
