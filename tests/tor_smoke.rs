use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use sentinel::proxy::socks::socks5_connect;

const TOR_SOCKS_ADDR: &str = "127.0.0.1:9050";

async fn tor_socks_port_listening() -> bool {
    tokio::time::timeout(
        Duration::from_millis(500),
        tokio::net::TcpStream::connect(TOR_SOCKS_ADDR),
    )
    .await
    .map(|r| r.is_ok())
    .unwrap_or(false)
}

#[tokio::test]
async fn tor_socks5_connect_reaches_the_internet() {
    if !tor_socks_port_listening().await {
        eprintln!(
            "[tor-test] skipped: no Tor SOCKS listener on {TOR_SOCKS_ADDR} \
             (start a local Tor daemon to exercise this integration test)"
        );
        return;
    }

    let mut stream = socks5_connect(TOR_SOCKS_ADDR, "example.com", 80)
        .await
        .expect("socks5 connect via tor failed");

    stream
        .write_all(b"GET / HTTP/1.0\r\nHost: example.com\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut resp = String::new();
    stream.read_to_string(&mut resp).await.unwrap();
    println!("[tor-test] got {} bytes", resp.len());
    assert!(resp.starts_with("HTTP/"), "unexpected response: {resp:.80}");
}
