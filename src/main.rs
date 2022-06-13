use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// https://github.com/tokio-rs/tokio
// https://rust-lang.github.io/async-book/
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080")
        .await
        .expect("TCP listener failed to start");
    let (socket, _addr) = listener.accept().await.unwrap();
}
