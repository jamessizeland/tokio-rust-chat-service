use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

// https://github.com/tokio-rs/tokio
// https://rust-lang.github.io/async-book/
#[tokio::main]
async fn main() {
    println!("starting Tokio Server");
    let listener = TcpListener::bind("localhost:8080")
        .await
        .expect("TCP listener failed to start");
    let (mut socket, _addr) = listener.accept().await.unwrap();

    let (reader, mut writer) = socket.split();

    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            println!("Exiting Program");
            break; // exit the loop gracefully if no data sent
        }

        writer.write_all(&line.as_bytes()).await.unwrap();
        line.clear(); // flush the input buffer before the next one
    }
}
