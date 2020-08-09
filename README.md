# async-http-client-lite

* [Cargo package](https://crates.io/crates/async-http-client-lite)

## Examples

### async_net 

* [http](demos/async_net/src/http.rs)
* [https](demos/async_net/src/https.rs)
* [https_with_https_proxy](demos/async_net/src/https_with_https_proxy.rs)
* [ws](demos/async_net/src/ws.rs)
* [ws_with_http_proxy](demos/async_net/src/ws_with_http_proxy.rs)
* [wss](demos/async_net/src/wss.rs)
* [wss_with_http_proxy](demos/async_net/src/wss_with_http_proxy.rs)

### async_std

* [wss_with_http_proxy](demos/async_std/src/wss_with_http_proxy.rs)

## Dev

```
cargo test --all-features --all -- --nocapture && \
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```

```
cargo build-all-features
cargo test-all-features --all
```

```
cargo tarpaulin --all-features
```
