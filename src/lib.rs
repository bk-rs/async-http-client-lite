pub mod client;
pub mod client_ext;
pub mod client_ext_ws;
pub mod client_http_tunnel;
pub mod client_proxy;
pub mod client_tls;

pub use client::Client;
pub use client_ext::ClientBackendKind;
pub use client_http_tunnel::{Authorization, ClientHttpTunnel, ClientHttpTunnelKind};
pub use client_proxy::ClientProxy;
pub use client_tls::{ClientTls, ClientTlsKind};

//
pub use http;
pub use http::{HeaderMap, HeaderValue, Request, Response, StatusCode, Version};

//
//
//
#[cfg(feature = "connector")]
pub mod connector;
#[cfg(feature = "connector")]
pub use connector::Connector;

#[cfg(feature = "connector__async_net")]
pub mod async_net_connector;
#[cfg(feature = "connector__async_net")]
pub use async_net_connector::AsyncNetTcpConnector;

#[cfg(all(feature = "connector__async_net", unix))]
pub use async_net_connector::AsyncNetUnixConnector;

#[cfg(feature = "connector__async_std")]
pub mod async_std_connector;
#[cfg(feature = "connector__async_std")]
pub use async_std_connector::AsyncStdTcpConnector;

#[cfg(all(feature = "connector__async_std", unix))]
pub use async_std_connector::AsyncStdUnixConnector;
