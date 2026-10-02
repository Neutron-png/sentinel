use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use tauri::{Emitter, Manager, State};
use tokio::sync::watch;
use uuid::Uuid;

use sentinel::db::repository::Repository;
use sentinel::history::models::HistoryEntry;
use sentinel::models::assessment::{
    Assessment, AssessmentStatus, Environment, Methodology, Scope,
};
use sentinel::models::finding::Finding;
use sentinel::proxy::config::ProxyConfig;
use sentinel::proxy::events::{EventBus, ProxyEvent};
use sentinel::proxy::server::ProxyServer;
use sentinel::proxy::socks::Upstream;

type DbRef = Arc<Mutex<Repository>>;

struct Db(DbRef);

#[derive(Debug, Clone)]
struct ProxySettings {
    port: u16,
    tor_enabled: bool,
    tor_addr: String,
}

impl Default for ProxySettings {
    fn default() -> Self {
        Self {
            port: 8080,
            tor_enabled: false,
            tor_addr: "127.0.0.1:9050".to_string(),
        }
    }
}

struct ProxyState {
    event_bus: Mutex<EventBus>,
    shutdown: Mutex<Option<watch::Sender<bool>>>,
    settings: Mutex<ProxySettings>,
    db: DbRef,
}

fn resolve_db_path() -> PathBuf {
    for p in ["sentinel.db", "../sentinel.db", "../../sentinel.db"] {
        let path = PathBuf::from(p);
        if path.exists() {
            return path;
        }
    }
    PathBuf::from("sentinel.db")
}

fn build_proxy_config(settings: &ProxySettings) -> ProxyConfig {
    ProxyConfig {
        listen_addr: SocketAddr::from(([127, 0, 0, 1], settings.port)),
        upstream: if settings.tor_enabled {
            Upstream::Socks5 {
                addr: settings.tor_addr.clone(),
                target_dns: true,
            }
        } else {
            Upstream::Direct
        },
        ..Default::default()
    }
}

fn start_proxy(state: &ProxyState, settings: &ProxySettings) -> Result<(), String> {
    // stop any previous instance
    if let Some(tx) = state.shutdown.lock().unwrap().take() {
        let _ = tx.send(true);
    }

    let mut server =
        ProxyServer::new(build_proxy_config(settings)).map_err(|e| format!("proxy init: {e}"))?;
    let bus = server.event_bus();
    *state.shutdown.lock().unwrap() = server.shutdown_tx_clone();

    // Run the server inside a task on tauri's runtime and keep `server` alive
    // for the lifetime of the app — a blocking `block_on(start())` would drop
    // the temporary runtime context and kill the accept loop.
    let (ready_tx, ready_rx) = tokio::sync::oneshot::channel::<Result<(), String>>();
    tauri::async_runtime::spawn(async move {
        match server.start().await {
            Ok(()) => {
                let _ = ready_tx.send(Ok(()));
                std::future::pending::<()>().await;
            }
            Err(e) => {
                let _ = ready_tx.send(Err(format!("proxy start: {e}")));
            }
        }
    });

    match tauri::async_runtime::block_on(async { ready_rx.await }) {
        Ok(Ok(())) => {}
        Ok(Err(e)) => return Err(e),
        Err(_) => return Err("proxy task died before binding".into()),
    }

    *state.event_bus.lock().unwrap() = bus;
    eprintln!("[proxy] started OK");
    Ok(())
}

#[tauri::command]
fn list_assessments(db: State<Db>) -> Result<Vec<Assessment>, String> {
    db.0.lock().unwrap().list_assessments().map_err(|e| e.to_string())
}

#[tauri::command]
fn create_assessment(
    db: State<Db>,
    name: String,
    target: String,
    environment: String,
    scope: String,
    methodology: String,
) -> Result<Assessment, String> {
    let environment = Environment::from_label(&environment)
        .ok_or_else(|| format!("unknown environment: {environment}"))?;
    let scope = Scope::from_label(&scope).ok_or_else(|| format!("unknown scope: {scope}"))?;
    let methodology = Methodology::from_label(&methodology)
        .ok_or_else(|| format!("unknown methodology: {methodology}"))?;

    let now = Utc::now();
    let assessment = Assessment {
        id: Uuid::new_v4(),
        name,
        target,
        environment,
        scope,
        methodology,
        status: AssessmentStatus::Draft,
        created_at: now,
        updated_at: now,
    };

    db.0.lock().unwrap().create_assessment(&assessment).map_err(|e| e.to_string())?;
    Ok(assessment)
}

#[tauri::command]
fn list_findings(db: State<Db>, assessment_id: String) -> Result<Vec<Finding>, String> {
    let id = Uuid::parse_str(&assessment_id).map_err(|e| e.to_string())?;
    db.0.lock().unwrap().list_findings_by_assessment(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_assessment(db: State<Db>, assessment_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&assessment_id).map_err(|e| e.to_string())?;
    db.0.lock().unwrap().delete_assessment(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_traffic(proxy: State<ProxyState>, limit: usize) -> Result<Vec<HistoryEntry>, String> {
    let limit = limit.clamp(1, 1000);
    proxy.db.lock().unwrap().list_history(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn proxy_status(proxy: State<ProxyState>) -> Result<serde_json::Value, String> {
    proxy_status_inner(&proxy)
}

pub fn http_client_config(proxy: &ProxyState) -> sentinel::network::config::HttpClientConfig {
    let settings = proxy.settings.lock().unwrap();
    let mut cfg = sentinel::network::config::HttpClientConfig::default();
    if settings.tor_enabled {
        cfg.proxy_url = Some(format!("socks5h://{}", settings.tor_addr));
        cfg.tls_verify = false;
    }
    cfg
}

#[derive(serde::Serialize)]
struct RepeaterResponse {
    status_code: u16,
    status_text: String,
    protocol: String,
    url: String,
    headers_text: String,
    body: String,
    duration_ms: u64,
}

#[tauri::command]
async fn repeater_send(
    proxy: State<'_, ProxyState>,
    raw_request: String,
) -> Result<RepeaterResponse, String> {
    let request =
        sentinel::repeater::parser::parse_raw_request(&raw_request).map_err(|e| e)?;
    let client = sentinel::network::client::HttpClient::new(http_client_config(&proxy))
        .map_err(|e| e.to_string())?;

    let started = std::time::Instant::now();
    let resp = client.execute(request).await.map_err(|e| format!("send failed: {e}"))?;
    let duration_ms = started.elapsed().as_millis() as u64;

    let headers_text = resp
        .headers
        .iter()
        .map(|h| format!("{}: {}", h.name, h.value))
        .collect::<Vec<_>>()
        .join("\r\n");
    let body_text = match &resp.body {
        sentinel::network::models::HttpBody::Text(s) => s.clone(),
        sentinel::network::models::HttpBody::Bytes(b) => String::from_utf8_lossy(b).to_string(),
        sentinel::network::models::HttpBody::Json(v) => v.to_string(),
        _ => String::new(),
    };

    // record in history so it shows in traffic + can be reused
    let entry = HistoryEntry {
        id: Uuid::new_v4(),
        transaction_id: None,
        timestamp: Utc::now(),
        method: request_method_of(&raw_request),
        scheme: resp.url.split("://").next().unwrap_or("http").to_string(),
        host: url_host_of(&resp.url),
        port: 0,
        path: String::new(),
        query: String::new(),
        url: resp.url.clone(),
        protocol: resp.protocol.clone(),
        status_code: resp.status_code,
        request_size: raw_request.len() as u64,
        response_size: body_text.len() as u64,
        duration_ms,
        tls_enabled: resp.url.starts_with("https://"),
        source: "repeater".into(),
        tags: String::new(),
        request_body: raw_request.clone(),
        response_body: String::new(),
        request_headers: String::new(),
        response_headers: headers_text.clone(),
        connection_id: None,
        stream_id: None,
        frame_metadata_json: None,
        negotiated_alpn: None,
        protocol_version: resp.protocol.clone(),
    };
    proxy.db.lock().unwrap().insert_history(&entry).map_err(|e| e.to_string())?;
    let _ = proxy.event_bus.lock().unwrap().emit(ProxyEvent::TransactionCaptured {
        method: entry.method.clone(),
        url: entry.url.clone(),
        host: entry.host.clone(),
        port: entry.port,
        status: resp.status_code,
        protocol: resp.protocol.clone(),
        tls_enabled: entry.tls_enabled,
        request_size: entry.request_size,
        response_size: entry.response_size,
        duration_ms,
        request_body: raw_request,
        response_body: headers_text.clone(),
        via_upstream: "repeater".into(),
    });

    Ok(RepeaterResponse {
        status_code: resp.status_code,
        status_text: resp.status_text,
        protocol: resp.protocol,
        url: resp.url,
        headers_text,
        body: body_text.chars().take(256 * 1024).collect(),
        duration_ms,
    })
}

fn request_method_of(raw: &str) -> String {
    raw.lines().next().unwrap_or("GET").split_whitespace().next().unwrap_or("GET").to_string()
}

fn url_host_of(url: &str) -> String {
    url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.to_string()))
        .unwrap_or_default()
}

#[tauri::command]
fn set_proxy_mode(
    proxy: State<ProxyState>,
    tor_enabled: bool,
    tor_addr: String,
) -> Result<serde_json::Value, String> {
    let mut s = proxy.settings.lock().unwrap();
    if tor_addr.trim().is_empty() {
        return Err("tor address cannot be empty".into());
    }
    s.tor_enabled = tor_enabled;
    s.tor_addr = tor_addr.trim().to_string();

    // restart proxy with the new upstream
    if let Some(tx) = proxy.shutdown.lock().unwrap().take() {
        let _ = tx.send(true);
    }
    let snapshot = s.clone();
    drop(s);
    start_proxy(&proxy, &snapshot)?;
    proxy_status_inner(&proxy)
}

fn proxy_status_inner(proxy: &ProxyState) -> Result<serde_json::Value, String> {
    let s = proxy.settings.lock().unwrap();
    Ok(serde_json::json!({
        "listening": true,
        "port": s.port,
        "tor_enabled": s.tor_enabled,
        "tor_addr": s.tor_addr,
    }))
}

fn event_to_history(e: &ProxyEvent) -> Option<HistoryEntry> {
    if let ProxyEvent::TransactionCaptured {
        method,
        url,
        host,
        port,
        status,
        protocol,
        tls_enabled,
        request_size,
        response_size,
        duration_ms,
        request_body,
        response_body,
        ..
    } = e
    {
        let parsed = url::Url::parse(url).ok();
        let path = parsed.as_ref().map(|u| u.path().to_string()).unwrap_or_default();
        let query = parsed.as_ref().map(|u| u.query().unwrap_or("").to_string()).unwrap_or_default();
        let scheme = parsed.as_ref().map(|u| u.scheme().to_string()).unwrap_or_default();

        Some(HistoryEntry {
            id: Uuid::new_v4(),
            transaction_id: None,
            timestamp: Utc::now(),
            method: method.clone(),
            scheme: if scheme.is_empty() { "https".into() } else { scheme },
            host: host.clone(),
            port: *port,
            path,
            query,
            url: url.clone(),
            protocol: protocol.clone(),
            status_code: *status,
            request_size: *request_size,
            response_size: *response_size,
            duration_ms: *duration_ms,
            tls_enabled: *tls_enabled,
            source: "proxy".into(),
            tags: String::new(),
            request_body: String::new(),
            response_body: String::new(),
            request_headers: request_body.clone(),
            response_headers: response_body.clone(),
            connection_id: None,
            stream_id: None,
            frame_metadata_json: None,
            negotiated_alpn: None,
            protocol_version: protocol.clone(),
        })
    } else {
        None
    }
}

mod scan;

fn main() {
    let repo = Arc::new(Mutex::new(
        Repository::open(&resolve_db_path()).expect("failed to open sentinel.db"),
    ));

    tauri::Builder::default()
        .manage(Db(repo.clone()))
        .setup(move |app| {
            let state = ProxyState {
                event_bus: Mutex::new(EventBus::new(1024)),
                shutdown: Mutex::new(None),
                settings: Mutex::new(ProxySettings::default()),
                db: repo.clone(),
            };

            let settings = state.settings.lock().unwrap().clone();
            if let Err(e) = start_proxy(&state, &settings) {
                eprintln!("[proxy] FAILED: {e}");
            }
            eprintln!("[app] setup continuing");

            app.manage(state);

            // forward captured transactions → DB + live frontend event
            let db = repo.clone();
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    let mut rx = handle
                        .state::<ProxyState>()
                        .event_bus
                        .lock()
                        .unwrap()
                        .subscribe();
                    loop {
                        match rx.recv().await {
                            Ok(ev) => {
                                if let Some(entry) = event_to_history(&ev) {
                                    if let Err(e) = db.lock().unwrap().insert_history(&entry) {
                                        eprintln!("history insert failed: {e}");
                                    }
                                    let _ = handle.emit("traffic-captured", &entry);
                                }
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                                eprintln!("proxy event bus lagged, dropped {n}");
                            }
                            Err(_) => break,
                        }
                    }
                    // channel closed (proxy restarted) → resubscribe on the new bus
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_assessments,
            create_assessment,
            list_findings,
            delete_assessment,
            list_traffic,
            proxy_status,
            set_proxy_mode,
            repeater_send,
            scan::start_scan
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
