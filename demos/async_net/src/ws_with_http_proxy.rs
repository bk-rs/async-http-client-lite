/*
cargo run -p async-http-client-lite-demo-async-net --bin ws_with_http_proxy echo.websocket.org 80 / 127.0.0.1 8118
*/
use std::env;
use std::io;

use async_tungstenite::tungstenite::protocol::Message;
use futures::executor::block_on;
use futures::{SinkExt, StreamExt};

use async_http_client_lite::{
    AsyncNetTcpConnector, ClientHttpTunnel, ClientHttpTunnelKind, ClientProxy, Connector,
};

fn main() -> io::Result<()> {
    block_on(run())
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("echo.websocket.org".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("80".to_owned()))
        .parse()
        .unwrap();
    let uri = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("URI").unwrap_or("/".to_owned()));
    let proxy_domain = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("PROXY_DOMAIN").unwrap_or("127.0.0.1".to_owned()));
    let proxy_port: u16 = env::args()
        .nth(5)
        .unwrap_or_else(|| env::var("PROXY_PORT").unwrap_or("8118".to_owned()))
        .parse()
        .unwrap();

    println!(
        "ws_with_http_proxy {} {} {} {} {}",
        domain, port, uri, proxy_domain, proxy_port
    );

    //
    let addr = format!("{}:{}", proxy_domain, proxy_port);

    //
    let client = AsyncNetTcpConnector::connect(
        addr,
        Some(ClientProxy::Http(ClientHttpTunnel::new(
            ClientHttpTunnelKind::AsyncHttp1Lite,
            domain.to_owned(),
            port,
            None,
            None,
        ))),
        None,
    )
    .await?;

    let (mut stream, response) = client
        .into_async_tungstenite(format!("ws://{}:{}", domain, port), None)
        .await
        .unwrap();

    println!("{:?}", response);

    stream
        .send(Message::Text("foo".to_owned()))
        .await
        .map_err(|err| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("write_message failed, err: {:?}", err),
            )
        })?;

    let msg = stream.next().await.unwrap().map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("read_message failed, err: {:?}", err),
        )
    })?;

    println!("{:?}", msg);

    println!("done");

    Ok(())
}
