use futures_io::{AsyncRead, AsyncWrite};

//
#[cfg(feature = "ws__async_tungstenite")]
use std::io;

#[cfg(feature = "ws__async_tungstenite")]
use http::Response;

//
#[cfg(feature = "ws__async_tungstenite")]
pub use async_tungstenite::tungstenite::protocol::WebSocketConfig as AsyncTungsteniteWebSocketConfig;
#[cfg(feature = "ws__async_tungstenite")]
use async_tungstenite::{
    client_async_with_config as async_tungstenite_client_async_with_config,
    tungstenite::{
        client::IntoClientRequest as AsyncTungsteniteIntoClientRequest,
        error::Error as AsyncTungsteniteWsError,
    },
    WebSocketStream as AsyncTungsteniteWebSocketStream,
};

//
use crate::client::Client;

//
//
//
impl<S> Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    #[cfg(feature = "ws__async_tungstenite")]
    pub async fn into_async_tungstenite<R>(
        self,
        request: R,
        config: Option<AsyncTungsteniteWebSocketConfig>,
    ) -> std::result::Result<
        (AsyncTungsteniteWebSocketStream<S>, Response<()>),
        AsyncTungsteniteWsError,
    >
    where
        R: AsyncTungsteniteIntoClientRequest + Unpin,
    {
        let request = request.into_client_request()?;
        match request.uri().scheme_str() {
            Some("wss") | Some("https") => {
                return Err(AsyncTungsteniteWsError::Io(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Please refer to https://github.com/bk-rs/async-http-client-lite/blob/master/demos/async_net/src/wss.rs",
                )))
            }
            _ => {}
        }

        let (stream, response) =
            async_tungstenite_client_async_with_config(request, self.into_inner(), config).await?;

        Ok((stream, response))
    }
}
