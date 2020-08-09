#[cfg(feature = "tls")]
pub(crate) use async_stream_tls_upgrader::UnionableTlsClientUpgrader;

#[cfg(feature = "tls__async_tls")]
pub use async_stream_tls_upgrader::async_tls_client::{
    ClientConfig as AsyncTlsRustlsClientConfig, TlsConnector as AsyncTlsTlsConnector,
    TLS_SERVER_ROOTS as ASYNC_TLS_WEBPKI_ROOTS_TLS_SERVER_ROOTS,
};
#[cfg(feature = "tls__async_tls")]
use async_stream_tls_upgrader::AsyncTlsClientTlsUpgrader;

#[cfg(feature = "tls__async_native_tls")]
pub use async_stream_tls_upgrader::async_native_tls_client::{
    Certificate as AsyncNativeTlsCertificate, Identity as AsyncNativeTlsIdentity,
    Protocol as AsyncNativeTlsProtocol, TlsConnector as AsyncNativeTlsTlsConnector,
};
#[cfg(feature = "tls__async_native_tls")]
use async_stream_tls_upgrader::AsyncNativeTlsClientTlsUpgrader;

//
//
//
pub struct ClientTls {
    #[allow(dead_code)]
    kind: ClientTlsKind,
    #[allow(dead_code)]
    domain: String,
}
impl ClientTls {
    pub fn new(kind: ClientTlsKind, domain: String) -> Self {
        Self { kind, domain }
    }

    #[cfg(feature = "tls")]
    pub(crate) fn into_tls_upgrader(self) -> UnionableTlsClientUpgrader {
        match self.kind {
            #[cfg(feature = "tls__async_tls")]
            ClientTlsKind::AsyncTls(tls_connector) => UnionableTlsClientUpgrader::AsyncTls(
                AsyncTlsClientTlsUpgrader::new(tls_connector, self.domain.to_owned()),
            ),
            #[cfg(feature = "tls__async_native_tls")]
            ClientTlsKind::AsyncNativeTls(tls_connector) => {
                UnionableTlsClientUpgrader::AsyncNativeTls(AsyncNativeTlsClientTlsUpgrader::new(
                    tls_connector,
                    self.domain,
                ))
            }
        }
    }
}

//
//
//
pub enum ClientTlsKind {
    #[cfg(feature = "tls__async_tls")]
    AsyncTls(AsyncTlsTlsConnector),
    #[cfg(feature = "tls__async_native_tls")]
    AsyncNativeTls(AsyncNativeTlsTlsConnector),
}
impl ClientTlsKind {
    #[cfg(feature = "tls__async_tls")]
    pub fn default_async_tls() -> Self {
        Self::AsyncTls(Default::default())
    }
    #[cfg(feature = "tls__async_native_tls")]
    pub fn default_async_native_tls() -> Self {
        Self::AsyncNativeTls(Default::default())
    }
}
