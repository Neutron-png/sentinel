use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use sentinel::network::client::HttpClient;
use sentinel::network::config::HttpClientConfig;
use sentinel::scanner::pipeline::executor::PipelineExecutor;
use sentinel::scanner::rules::active::sqli::rule::SqliRule;
use sentinel::scanner::sdk::result::RuleStatus;

const NORMAL_BODY: &str = "<html><body>item 1</body></html>";
const ERROR_BODY: &str =
    "<html><body>You have an error in your SQL syntax; check the manual near '' at line 1</body></html>";

fn spawn_server(vulnerable: bool) -> std::net::SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::from_std(listener).unwrap();
            loop {
                let Ok((mut sock, _)) = listener.accept().await else {
                    return;
                };
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 8192];
                    let n = match sock.read(&mut buf).await {
                        Ok(0) | Err(_) => return,
                        Ok(n) => n,
                    };
                    let request = String::from_utf8_lossy(&buf[..n]);
                    let target = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                        .unwrap_or("/");
                    let injects_quote = target.contains("%27") || target.contains('\'');
                    let (status, body) = if vulnerable && injects_quote {
                        ("500 Internal Server Error", ERROR_BODY)
                    } else {
                        ("200 OK", NORMAL_BODY)
                    };
                    let response = format!(
                        "HTTP/1.1 {status}\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    );
                    let _ = sock.write_all(response.as_bytes()).await;
                });
            }
        });
    });
    addr
}

async fn run_sqli_scan(addr: std::net::SocketAddr) -> Vec<sentinel::scanner::sdk::result::RuleResult> {
    let client = Arc::new(HttpClient::new(HttpClientConfig::default()).unwrap());
    let rule = SqliRule::new();
    let executor = PipelineExecutor::new();
    let (cancel_tx, _cancel_rx) = tokio::sync::watch::channel(false);
    let target = format!("http://{}", addr);
    let (results, _progress) = executor
        .execute(&rule, client, &target, cancel_tx)
        .await;
    results
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn error_based_sqli_is_detected_on_vulnerable_fixture() {
    let addr = spawn_server(true);
    tokio::time::sleep(Duration::from_millis(100)).await;
    let results = run_sqli_scan(addr).await;
    assert!(
        results.iter().any(|r| r.title.contains("Error-Based")),
        "expected an error-based SQLi finding, got: {:?}",
        results.iter().map(|r| r.title.clone()).collect::<Vec<_>>()
    );
    assert!(
        results.iter().all(|r| r.status == RuleStatus::Potential),
        "every SQLi result must be Potential, never Confirmed, without independent verification"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn safe_fixture_produces_zero_sqli_findings() {
    let addr = spawn_server(false);
    tokio::time::sleep(Duration::from_millis(100)).await;
    let results = run_sqli_scan(addr).await;
    assert!(
        results.is_empty(),
        "a safe application must yield zero SQLi findings, got: {:?}",
        results.iter().map(|r| r.title.clone()).collect::<Vec<_>>()
    );
}
