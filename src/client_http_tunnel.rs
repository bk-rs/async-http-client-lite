pub use async_stream_http_tunnel_grader::Authorization;
use http::{HeaderMap, HeaderValue};

//
#[cfg(feature = "http_tunnel")]
pub(crate) use async_stream_http_tunnel_grader::UnionableHttpTunnelClientGrader;

#[cfg(feature = "http_tunnel__async_http1_lite")]
use async_stream_http_tunnel_grader::AsyncHttp1LiteClientHttpTunnelGrader;

//
//
//
pub struct ClientHttpTunnel {
    #[allow(dead_code)]
    kind: ClientHttpTunnelKind,
    #[allow(dead_code)]
    remote_host: String,
    #[allow(dead_code)]
    remote_port: u16,
    #[allow(dead_code)]
    proxy_authorization: Option<Authorization>,
    #[allow(dead_code)]
    proxy_headers: Option<HeaderMap<HeaderValue>>,
}
impl ClientHttpTunnel {
    pub fn new(
        kind: ClientHttpTunnelKind,
        remote_host: String,
        remote_port: u16,
        proxy_authorization: Option<Authorization>,
        proxy_headers: Option<HeaderMap<HeaderValue>>,
    ) -> Self {
        Self {
            kind,
            remote_host,
            remote_port,
            proxy_authorization,
            proxy_headers,
        }
    }

    #[cfg(feature = "http_tunnel")]
    pub(crate) fn into_http_tunnel_grader(self) -> UnionableHttpTunnelClientGrader {
        match self.kind {
            #[cfg(feature = "http_tunnel__async_http1_lite")]
            ClientHttpTunnelKind::AsyncHttp1Lite => {
                UnionableHttpTunnelClientGrader::AsyncHttp1Lite(
                    AsyncHttp1LiteClientHttpTunnelGrader::new(
                        self.remote_host.to_owned(),
                        self.remote_port,
                        self.proxy_authorization,
                        self.proxy_headers,
                    ),
                )
            }
        }
    }
}

//
//
//
pub enum ClientHttpTunnelKind {
    #[cfg(feature = "http_tunnel__async_http1_lite")]
    AsyncHttp1Lite,
}
