use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use sentinel::proxy::config::ProxyConfig;
use sentinel::proxy::events::ProxyEvent;
use sentinel::proxy::server::ProxyServer;

#[tokio::test]
async fn proxy_forwards_plain_http_and_captures() {
    let addr: std::net::SocketAddr = "127.0.0.1:8188".parse().unwrap();
    let mut server = ProxyServer::new(ProxyConfig {
        listen_addr: addr,
        ..Default::default()
    })
    .unwrap();
    let bus = server.event_bus();
    let mut rx = bus.subscribe();
    server.start().await.unwrap();

    // pretend upstream: a tiny HTTP server
    let upstream = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8199").await.unwrap();
        loop {
            let Ok((mut sock, _)) = listener.accept().await else { return };
            tokio::spawn(async move {
                let mut buf = [0u8; 8192];
                let n = sock.read(&mut buf).await.unwrap_or(0);
                let req = String::from_utf8_lossy(&buf[..n]);
                let is_get = req.starts_with("GET");
                let resp = if is_get {
                    "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello"
                } else {
                    "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"
                };
                let _ = sock.write_all(resp.as_bytes()).await;
            });
        }
    });

    tokio::time::sleep(Duration::from_millis(200)).await;

    // sanity: upstream answers directly
    let direct = reqwest::Client::new()
        .get("http://127.0.0.1:8199/hello")
        .timeout(Duration::from_secs(5))
        .send()
        .await;
    println!("[test] direct upstream: {:?}", direct.as_ref().map(|r| r.status()));
    assert!(direct.is_ok(), "upstream not reachable directly");

    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:8188").unwrap())
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    println!("[test] sending through proxy...");
    let resp = client.get("http://127.0.0.1:8199/hello").send().await.unwrap();
    println!("[test] got response: {}", resp.status());
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.text().await.unwrap(), "hello");

    // wait for capture event
    let deadline = tokio::time::Instant::now() + Duration::from_secs(3);
    let mut captured = false;
    while tokio::time::Instant::now() < deadline {
        match rx.try_recv() {
            Ok(ProxyEvent::TransactionCaptured { status, .. }) => {
                assert_eq!(status, 200);
                captured = true;
                break;
            }
            Ok(_) => continue,
            Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Err(_) => break,
        }
    }
    assert!(captured, "no TransactionCaptured event received");
    upstream.abort();
}
