/*
cargo run -p async-http-client-lite-demo-async-net --bin https httpbin.org 443 /ip
*/

use std::env;
use std::io;

use futures::executor::block_on;

use async_http_client_lite::{
    AsyncNetTcpConnector, ClientBackendKind, ClientTls, ClientTlsKind, Connector, Request,
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

    println!("https {} {} {}", domain, port, uri);

    //
    let addr = format!("{}:{}", domain, port);

    //
    let mut client = AsyncNetTcpConnector::connect(
        addr,
        None,
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
