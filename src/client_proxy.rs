use crate::client_http_tunnel::ClientHttpTunnel;
use crate::client_tls::ClientTls;

pub enum ClientProxy {
    Http(ClientHttpTunnel),
    Https(ClientTls, ClientHttpTunnel),
}
