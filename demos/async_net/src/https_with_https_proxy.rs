/*
cargo run -p async-http-client-lite-demo-async-net --bin https_with_https_proxy httpbin.org 443 /ip proxy.lvh.me 9118
*/
use std::env;
use std::io;

use futures::executor::block_on;

use async_http_client_lite::{
    client_tls::AsyncNativeTlsTlsConnector, AsyncNetTcpConnector, ClientBackendKind,
    ClientHttpTunnel, ClientHttpTunnelKind, ClientProxy, ClientTls, ClientTlsKind, Connector,
    Request,
};

fn main() -> io::Result<()> {
    block_on(run())
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("httpbin.org".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("443".to_owned()))
        .parse()
        .unwrap();
    let uri = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("URI").unwrap_or("/ip".to_owned()));
    let proxy_domain = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("PROXY_DOMAIN").unwrap_or("proxy.lvh.me".to_owned()));
    let proxy_port: u16 = env::args()
        .nth(5)
        .unwrap_or_else(|| env::var("PROXY_PORT").unwrap_or("9118".to_owned()))
        .parse()
        .unwrap();

    println!(
        "https_with_https_proxy {} {} {} {} {}",
        domain, port, uri, proxy_domain, proxy_port
    );

    //
    let addr = format!("{}:{}", proxy_domain, proxy_port);

    //
    let tls_connector = AsyncNativeTlsTlsConnector::new().danger_accept_invalid_certs(true);

    let mut client = AsyncNetTcpConnector::connect(
        addr,
        Some(ClientProxy::Https(
            ClientTls::new(ClientTlsKind::AsyncNativeTls(tls_connector), proxy_domain),
            ClientHttpTunnel::new(
                ClientHttpTunnelKind::AsyncHttp1Lite,
                domain.to_owned(),
                port,
                None,
                None,
            ),
        )),
        Some(ClientTls::new(
            ClientTlsKind::default_async_tls(),
            domain.to_owned(),
        )),
    )
    .await?;

    //
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header("Host", domain)
        .header("User-Agent", "curl/7.71.1")
        .header("Accept", "*/*")
        .body(vec![])
        .unwrap();
    println!("{:?}", request);

    let (response, _) = client
        .response(request, ClientBackendKind::H1AsyncHttp1Lite)
        .await?;

    println!("{:?}", response);

    println!("done");

    Ok(())
}
