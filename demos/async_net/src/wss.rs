/*
cargo run -p async-http-client-lite-demo-async-net --bin wss echo.websocket.org 443 /
*/

use std::env;
use std::io;

use async_tungstenite::tungstenite::protocol::Message;
use futures::executor::block_on;
use futures::{SinkExt, StreamExt};

use async_http_client_lite::{AsyncNetTcpConnector, ClientTls, ClientTlsKind, Connector};

fn main() -> io::Result<()> {
    block_on(run())
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("echo.websocket.org".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("443".to_owned()))
        .parse()
        .unwrap();
    let uri = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("URI").unwrap_or("/".to_owned()));

    println!("wss {} {} {}", domain, port, uri);

    //
    let addr = format!("{}:{}", domain, port);

    //
    let client = AsyncNetTcpConnector::connect(
        addr,
        None,
        Some(ClientTls::new(
            ClientTlsKind::default_async_native_tls(),
            domain.to_owned(),
        )),
    )
    .await?;

    //
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
