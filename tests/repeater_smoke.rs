use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use sentinel::network::client::HttpClient;
use sentinel::repeater::parser::parse_raw_request;

#[tokio::test]
async fn repeater_parses_and_sends_real_request() {
    // fake upstream
    let server = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8201").await.unwrap();
        let (mut sock, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 8192];
        let n = sock.read(&mut buf).await.unwrap_or(0);
        let req = String::from_utf8_lossy(&buf[..n]).to_string();
        let resp = if req.contains("POST /echo") && req.contains("body=hello") {
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 12\r\n\r\necho-OK-done"
        } else {
            "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n"
        };
        let _ = sock.write_all(resp.as_bytes()).await;
    });

    tokio::time::sleep(Duration::from_millis(150)).await;

    let raw = "POST /echo HTTP/1.1\r\nHost: 127.0.0.1:8201\r\nContent-Length: 10\r\n\r\nbody=hello";
    let request = parse_raw_request(raw).expect("parse failed");
    assert_eq!(request.method, "POST");

    let client = HttpClient::new(Default::default()).unwrap();
    let resp = client.execute(request).await.expect("execute failed");
    assert_eq!(resp.status_code, 200);
    let body = resp.body.as_text().unwrap_or_default().to_string();
    assert_eq!(body, "echo-OK-done");

    server.abort();
}
