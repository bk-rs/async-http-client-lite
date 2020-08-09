use std::io;

use futures_io::{AsyncRead, AsyncWrite};
use http::{Request, Response, Version};

//
#[cfg(feature = "h1__async_http1_lite")]
use async_http1_lite::Http1ClientStream as AsyncHttp1LiteHttp1ClientStream;

//
use crate::client::Client;

//
//
//
pub enum ClientBackendKind {
    #[cfg(feature = "h1__async_http1_lite")]
    H1AsyncHttp1Lite,
}

impl<S> Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    pub async fn response(
        &mut self,
        request: Request<Vec<u8>>,
        backend_kind: ClientBackendKind,
    ) -> io::Result<(Response<Vec<u8>>, Option<Vec<u8>>)> {
        if request.version() <= Version::HTTP_11 {
            match backend_kind {
                #[cfg(feature = "h1__async_http1_lite")]
                ClientBackendKind::H1AsyncHttp1Lite => {
                    let mut stream = AsyncHttp1LiteHttp1ClientStream::new(self.get_mut());
                    stream.write_request(request).await?;

                    let (response, reason_phrase) = stream.read_response().await?;

                    Ok((response, reason_phrase))
                }
            }
        } else if request.version() <= Version::HTTP_2 {
            // TODO, `curl -x https://proxy.lvh.me:9118 https://httpbin.org/ip -v --proxy-insecure` is very slow, maybe warning it is better.
            return Err(io::Error::new(io::ErrorKind::Other, "unimplemented now"));
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "unimplemented now"));
        }
    }
}

//
//
//
impl<S> Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    #[cfg(feature = "h1__async_http1_lite")]
    pub fn into_async_http1_lite(self) -> io::Result<AsyncHttp1LiteHttp1ClientStream<S>> {
        Ok(AsyncHttp1LiteHttp1ClientStream::new(self.into_inner()))
    }
}
