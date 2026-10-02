#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::sync::{watch, Semaphore};

use crate::intruder::errors::IntruderError;
use crate::intruder::models::{
    AnalysisBaseline, AttackMode, InsertionPoint, IntruderConfig, ResponseMetrics,
};
use crate::intruder::payloads::{apply_encoding, expand};
use crate::network::client::HttpClient;
use crate::network::models::HttpRequest;

pub type ScopeCheck = Arc<dyn Fn(&str) -> bool + Send + Sync + 'static>;

#[derive(Debug, Clone, Copy)]
pub struct RateGate {
    interval: Duration,
}

impl RateGate {
    pub fn new(rate_per_sec: f64) -> Self {
        let interval = if rate_per_sec <= 0.0 {
            Duration::ZERO
        } else {
            Duration::from_secs_f64(1.0 / rate_per_sec)
        };
        Self { interval }
    }

    pub async fn acquire(&self, slot: &Mutex<Instant>) {
        if self.interval == Duration::ZERO {
            return;
        }
        loop {
            let wait = {
                let mut next_free = slot.lock().unwrap();
                let now = Instant::now();
                if now >= *next_free {
                    *next_free = now + self.interval;
                    Duration::ZERO
                } else {
                    let wait = *next_free - now;
                    *next_free += self.interval;
                    wait
                }
            };
            if wait == Duration::ZERO {
                return;
            }
            tokio::time::sleep(wait).await;
        }
    }
}

pub const POSITION_OPEN: char = '\u{a7}';

#[derive(Debug, Clone, PartialEq)]
pub enum TemplatePart {
    Literal(String),
    Slot(usize),
}

#[derive(Debug, Clone)]
pub struct Template {
    pub parts: Vec<TemplatePart>,
    pub raw: String,
    pub slot_count: usize,
    pub positions: Vec<InsertionPoint>,
}

pub fn parse_template(raw: &str) -> Result<Template, IntruderError> {
    let marker = POSITION_OPEN.to_string();
    let mut parts: Vec<TemplatePart> = Vec::new();
    let mut positions = Vec::new();
    let mut offset = 0usize;

    let mut rest = raw;
    while let Some(start) = rest.find(&marker) {
        let (head, tail) = rest.split_at(start);
        let after = &tail[marker.len()..];
        let Some(end) = after.find(&marker) else {
            return Err(IntruderError::InvalidTemplate(
                "unterminated payload position".into(),
            ));
        };
        let payload = &after[..end];
        if payload.is_empty() {
            return Err(IntruderError::InvalidTemplate("empty payload position".into()));
        }
        if !head.is_empty() {
            parts.push(TemplatePart::Literal(head.to_string()));
            offset += head.len();
        }
        positions.push(InsertionPoint {
            start: offset,
            end: offset + payload.chars().count(),
        });
        parts.push(TemplatePart::Slot(positions.len() - 1));
        offset += payload.len();
        rest = &after[end + marker.len()..];
    }
    if !rest.is_empty() {
        parts.push(TemplatePart::Literal(rest.to_string()));
    }
    if positions.is_empty() {
        return Err(IntruderError::NoPositions);
    }
    Ok(Template {
        parts,
        raw: raw.to_string(),
        slot_count: positions.len(),
        positions,
    })
}

pub fn materialize(template: &Template, payloads: &[String]) -> String {
    let mut out = String::with_capacity(template.raw.len() + 32);
    for part in &template.parts {
        match part {
            TemplatePart::Literal(l) => out.push_str(l),
            TemplatePart::Slot(i) => {
                if let Some(p) = payloads.get(*i) {
                    out.push_str(p);
                }
            }
        }
    }
    out
}

pub fn build_plan(
    config: &IntruderConfig,
    template: &Template,
) -> Result<Vec<Vec<String>>, IntruderError> {
    if config.sets.len() != template.slot_count {
        return Err(IntruderError::InvalidPayloadSet(format!(
            "template has {} payload positions but {} payload sets provided",
            template.slot_count,
            config.sets.len()
        )));
    }
    let expansion_limit = usize::try_from(config.max_requests).unwrap_or(usize::MAX);
    let mut per_slot: Vec<Vec<String>> = Vec::with_capacity(template.slot_count);
    for set in &config.sets {
        let payloads = expand(set, expansion_limit)?;
        if payloads.is_empty() {
            return Err(IntruderError::InvalidPayloadSet(
                "payload set expands to zero items".into(),
            ));
        }
        per_slot.push(
            payloads
                .iter()
                .map(|p| apply_encoding(p, config.encoding))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    let combos: Vec<Vec<String>> = match config.mode {
        AttackMode::Sniper => {
            let mut combos = Vec::new();
            for slot in 0..template.slot_count {
                let mut row: Vec<String> = vec![String::new(); template.slot_count];
                for payload in &per_slot[slot] {
                    row[slot] = payload.clone();
                    combos.push(row.clone());
                }
            }
            combos
        }
        AttackMode::Pitchfork => {
            let set_len = per_slot.iter().map(|s| s.len()).max().unwrap_or(0);
            (0..set_len)
                .map(|i| {
                    per_slot
                        .iter()
                        .map(|s| s[i.min(s.len() - 1)].clone())
                        .collect()
                })
                .collect()
        }
        AttackMode::ClusterBomb => {
            let mut combos: Vec<Vec<String>> = vec![Vec::new()];
            for slot_payloads in &per_slot {
                let mut next = Vec::new();
                for existing in &combos {
                    for payload in slot_payloads {
                        let mut row = existing.clone();
                        row.push(payload.clone());
                        next.push(row);
                    }
                }
                combos = next;
            }
            combos
        }
    };

    let total = combos.len() as u64;
    if total > config.max_requests {
        return Err(IntruderError::PayloadCountExceeded {
            planned: total,
            cap: config.max_requests,
        });
    }
    Ok(combos)
}

pub struct FuzzRunSummary {
    pub results: Vec<crate::intruder::models::FuzzResult>,
    pub baseline: Option<AnalysisBaseline>,
    pub sent: u64,
    pub skipped_scope: u64,
    pub cancelled: bool,
    pub parse_errors: Vec<String>,
}

pub struct IntruderEngine {
    client: Arc<HttpClient>,
    scope_check: ScopeCheck,
    cancel_tx: Arc<watch::Sender<bool>>,
    cancel_rx: Arc<Mutex<watch::Receiver<bool>>>,
    running: Arc<AtomicBool>,
    sent: Arc<AtomicU64>,
}

impl IntruderEngine {
    pub fn new(client: Arc<HttpClient>) -> Self {
        Self::with_scope(client, Arc::new(|_: &str| true))
    }

    pub fn with_scope(client: Arc<HttpClient>, scope_check: ScopeCheck) -> Self {
        let (tx, rx) = watch::channel(false);
        Self {
            client,
            scope_check,
            cancel_tx: Arc::new(tx),
            cancel_rx: Arc::new(Mutex::new(rx)),
            running: Arc::new(AtomicBool::new(false)),
            sent: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn cancel(&self) {
        let _ = self.cancel_tx.send(true);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn sent(&self) -> u64 {
        self.sent.load(Ordering::SeqCst)
    }

    pub async fn run(
        &self,
        raw_template: &str,
        config: &IntruderConfig,
    ) -> Result<FuzzRunSummary, IntruderError> {
        if self
            .running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(IntruderError::NotRunning);
        }
        let _guard = RunGuard {
            flag: Arc::clone(&self.running),
        };
        self.sent.store(0, Ordering::SeqCst);
        let cancel_rx = watch::Receiver::clone(&self.cancel_rx.lock().unwrap());

        let template = parse_template(raw_template)?;
        let plan = build_plan(config, &template)?;
        let gate = Arc::new(RateGate::new(config.rate_limit_rps));
        let gate_slot = Arc::new(Mutex::new(Instant::now()));
        let semaphore = Arc::new(Semaphore::new(config.concurrency.max(1)));
        let scope = Arc::clone(&self.scope_check);
        let client = Arc::clone(&self.client);

        let mut handles = Vec::with_capacity(plan.len());
        let mut skipped_scope = 0u64;
        let mut parse_errors: Vec<String> = Vec::new();

        for (index, combo) in plan.into_iter().enumerate() {
            if *cancel_rx.borrow() {
                break;
            }
            let request_text = materialize(&template, &combo);
            if let Err(e) = validate_template_request(&request_text) {
                parse_errors.push(format!("entry {index}: {e}"));
                break;
            }
            let Some(probe) = probe_url(&request_text) else {
                parse_errors.push(format!("entry {index}: cannot resolve target"));
                break;
            };
            if !scope(&probe) {
                skipped_scope += 1;
                continue;
            }
            let handle = spawn_fuzz_task(
                Arc::clone(&client),
                request_text,
                combo,
                index as u64,
                Arc::clone(&semaphore),
                Arc::clone(&gate),
                Arc::clone(&gate_slot),
                watch::Receiver::clone(&cancel_rx),
                Arc::clone(&self.sent),
                config.request_timeout,
            );
            handles.push(handle);
        }

        let mut results = Vec::new();
        let mut run_cancelled = false;
        for handle in handles {
            match handle.await {
                Ok(Ok(r)) => results.push(r),
                Ok(Err(IntruderError::Cancelled)) => {
                    run_cancelled = true;
                }
                Ok(Err(e)) => return Err(e),
                Err(join_err) => return Err(IntruderError::Client(join_err.to_string())),
            }
        }
        run_cancelled = run_cancelled || *cancel_rx.borrow();

        results.sort_by_key(|r| r.index);
        let metrics: Vec<ResponseMetrics> = results.iter().map(|r| r.metrics.clone()).collect();
        let baseline = AnalysisBaseline::from_results(&metrics);
        for r in &mut results {
            r.anomaly = baseline.is_anomaly(&r.metrics);
        }

        Ok(FuzzRunSummary {
            results,
            baseline: Some(baseline),
            sent: self.sent(),
            skipped_scope,
            cancelled: run_cancelled,
            parse_errors,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_fuzz_task(
    client: Arc<HttpClient>,
    request_text: String,
    payloads: Vec<String>,
    index: u64,
    semaphore: Arc<Semaphore>,
    gate: Arc<RateGate>,
    gate_slot: Arc<Mutex<Instant>>,
    cancel_rx: watch::Receiver<bool>,
    sent_counter: Arc<AtomicU64>,
    timeout: Duration,
) -> tokio::task::JoinHandle<Result<crate::intruder::models::FuzzResult, IntruderError>> {
    tokio::spawn(async move {
        let _permit = semaphore.acquire().await;
        if *cancel_rx.borrow() {
            return Err(IntruderError::Cancelled);
        }
        gate.acquire(&gate_slot).await;
        if *cancel_rx.borrow() {
            return Err(IntruderError::Cancelled);
        }
        let request: HttpRequest = crate::repeater::parser::parse_raw_request(&request_text)
            .map_err(IntruderError::InvalidTemplate)?;
        let started = Instant::now();
        let response = match tokio::time::timeout(timeout, client.execute(request)).await {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => return Err(IntruderError::Client(e.to_string())),
            Err(_) => return Err(IntruderError::Client("request timed out".into())),
        };
        sent_counter.fetch_add(1, Ordering::SeqCst);
        let body_text = match &response.body {
            crate::network::models::HttpBody::Json(v) => v.to_string(),
            crate::network::models::HttpBody::Text(s) => s.clone(),
            other => String::from_utf8_lossy(other.as_bytes()).to_string(),
        };
        let metrics = ResponseMetrics {
            status_code: response.status_code,
            body_length: body_text.len(),
            word_count: body_text.split_whitespace().count(),
            line_count: body_text.lines().count(),
            duration: started.elapsed(),
        };
        Ok(crate::intruder::models::FuzzResult {
            index,
            payloads,
            metrics,
            anomaly: false,
        })
    })
}

fn validate_template_request(raw: &str) -> Result<(), String> {
    let mut lines = raw.lines();
    let start = lines.next().ok_or("empty request")?;
    let mut parts = start.split_whitespace();
    let _method = parts.next().ok_or("missing method")?;
    let _target = parts.next().ok_or("missing request target")?;
    let mut has_host = false;
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        if let Some((name, _)) = line.split_once(':') {
            if name.trim().eq_ignore_ascii_case("host") {
                has_host = true;
            }
        }
    }
    if !start.contains("http://") && !start.contains("https://") && !has_host {
        return Err("template must use an absolute-form URL or set a Host header".into());
    }
    Ok(())
}

fn probe_url(raw: &str) -> Option<String> {
    let mut lines = raw.lines();
    let start = lines.next()?;
    let mut parts = start.split_whitespace();
    parts.next()?;
    let target = parts.next()?;
    if target.starts_with("http://") || target.starts_with("https://") {
        return Some(target.to_string());
    }
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.trim().eq_ignore_ascii_case("host") {
                let host = value.trim();
                let scheme = if host.ends_with(":443") { "https" } else { "http" };
                return Some(format!("{scheme}://{host}{target}"));
            }
        }
    }
    None
}

struct RunGuard {
    flag: Arc<AtomicBool>,
}

impl Drop for RunGuard {
    fn drop(&mut self) {
        self.flag.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intruder::models::{AttackMode, PayloadEncoding, PayloadSet};
    use crate::network::config::HttpClientConfig;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    fn config(mode: AttackMode, sets: Vec<PayloadSet>) -> IntruderConfig {
        IntruderConfig {
            mode,
            sets,
            encoding: PayloadEncoding::None,
            ..Default::default()
        }
    }

    const TEMPLATE_2: &str = "GET /\u{a7}1\u{a7}/\u{a7}2\u{a7} HTTP/1.1\r\nHost: h\r\n";

    #[test]
    fn template_parse_and_materialize() {
        let t = parse_template("GET /\u{a7}id\u{a7}/x?q=\u{a7}q\u{a7} HTTP/1.1\r\nHost: h\r\n").unwrap();
        assert_eq!(t.slot_count, 2);
        let out = materialize(&t, &[String::from("A"), String::from("B")]);
        assert!(out.starts_with("GET /A/x?q=B HTTP/1.1"));
    }

    #[test]
    fn template_rejects_bad_positions() {
        assert!(parse_template("GET /\u{a7}a HTTP/1.1").is_err());
        assert!(parse_template("GET /\u{a7}\u{a7} HTTP/1.1").is_err());
        assert!(parse_template("GET / HTTP/1.1").is_err());
    }

    #[test]
    fn sniper_plan() {
        let cfg = config(
            AttackMode::Sniper,
            vec![
                PayloadSet::List(vec!["a".into(), "b".into()]),
                PayloadSet::List(vec!["c".into()]),
            ],
        );
        let t = parse_template(TEMPLATE_2).unwrap();
        let plan = build_plan(&cfg, &t).unwrap();
        assert_eq!(plan.len(), 3);
        assert_eq!(plan[0], vec!["a".to_string(), String::new()]);
        assert_eq!(plan[1], vec!["b".to_string(), String::new()]);
        assert_eq!(plan[2], vec![String::new(), "c".to_string()]);
    }

    #[test]
    fn cluster_bomb_plan() {
        let cfg = config(
            AttackMode::ClusterBomb,
            vec![
                PayloadSet::List(vec!["1".into(), "2".into()]),
                PayloadSet::List(vec!["x".into(), "y".into()]),
            ],
        );
        let t = parse_template(TEMPLATE_2).unwrap();
        let plan = build_plan(&cfg, &t).unwrap();
        assert_eq!(plan.len(), 4);
        assert!(plan.contains(&vec!["1".to_string(), "x".to_string()]));
        assert!(plan.contains(&vec!["2".to_string(), "y".to_string()]));
    }

    #[test]
    fn pitchfork_plan() {
        let cfg = config(
            AttackMode::Pitchfork,
            vec![
                PayloadSet::List(vec!["1".into(), "2".into()]),
                PayloadSet::List(vec!["x".into(), "y".into(), "z".into()]),
            ],
        );
        let t = parse_template(TEMPLATE_2).unwrap();
        let plan = build_plan(&cfg, &t).unwrap();
        assert_eq!(plan.len(), 3);
        assert_eq!(plan[0], vec!["1".to_string(), "x".to_string()]);
        assert_eq!(plan[2], vec!["2".to_string(), "z".to_string()]);
    }

    #[test]
    fn plan_respects_max_requests_cap() {
        let cfg = IntruderConfig {
            max_requests: 1,
            ..config(
                AttackMode::ClusterBomb,
                vec![
                    PayloadSet::List(vec!["1".into(), "2".into()]),
                    PayloadSet::List(vec!["x".into(), "y".into()]),
                ],
            )
        };
        let t = parse_template(TEMPLATE_2).unwrap();
        let err = build_plan(&cfg, &t).unwrap_err();
        assert!(matches!(err, IntruderError::PayloadCountExceeded { planned: 4, cap: 1 }));
    }

    #[test]
    fn plan_requires_matching_set_count() {
        let cfg = config(AttackMode::Sniper, vec![PayloadSet::List(vec!["a".into()])]);
        let t = parse_template(TEMPLATE_2).unwrap();
        assert!(build_plan(&cfg, &t).is_err());
    }

    #[tokio::test]
    async fn run_skips_out_of_scope() {
        let client = Arc::new(HttpClient::new(HttpClientConfig::default()).unwrap());
        let scope: ScopeCheck = Arc::new(|url| url.contains("allowed.example"));
        let engine = IntruderEngine::with_scope(client, scope);
        let cfg = IntruderConfig {
            mode: AttackMode::Sniper,
            sets: vec![PayloadSet::List(vec!["a".into(), "b".into()])],
            ..Default::default()
        };
        let summary = engine
            .run("GET /\u{a7}x\u{a7} HTTP/1.1\r\nHost: forbidden.example\r\n", &cfg)
            .await
            .unwrap();
        assert_eq!(summary.skipped_scope, 2);
        assert_eq!(summary.results.len(), 0);
        assert_eq!(summary.sent, 0);
    }

    #[tokio::test]
    async fn run_cancellation_stops_before_sending() {
        let client = Arc::new(HttpClient::new(HttpClientConfig::default()).unwrap());
        let engine = IntruderEngine::new(client);
        engine.cancel();
        let cfg = IntruderConfig {
            mode: AttackMode::Sniper,
            sets: vec![PayloadSet::List(vec!["a".into(), "b".into()])],
            ..Default::default()
        };
        let summary = engine
            .run("GET /\u{a7}x\u{a7} HTTP/1.1\r\nHost: forbidden.example\r\n", &cfg)
            .await
            .unwrap();
        assert!(summary.cancelled);
        assert_eq!(summary.sent, 0);
    }

    #[tokio::test]
    async fn run_against_local_server() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            loop {
                let (mut sock, _) = match listener.accept().await {
                    Ok(s) => s,
                    Err(_) => break,
                };
                tokio::spawn(async move {
                    let mut buf = [0u8; 4096];
                    loop {
                        let n = match sock.read(&mut buf).await {
                            Ok(0) | Err(_) => break,
                            Ok(n) => n,
                        };
                        let req = String::from_utf8_lossy(&buf[..n]);
                        let status = if req.contains("admin") { 403 } else { 200 };
                        let resp = format!(
                            "HTTP/1.1 {status} T\r\nContent-Length: 6\r\nConnection: close\r\n\r\nSTATUS"
                        );
                        if sock.write_all(resp.as_bytes()).await.is_err() {
                            break;
                        }
                        break;
                    }
                });
            }
        });

        let client = Arc::new(HttpClient::new(HttpClientConfig::default()).unwrap());
        let engine = IntruderEngine::new(client);
        let cfg = IntruderConfig {
            mode: AttackMode::Sniper,
            concurrency: 1,
            sets: vec![PayloadSet::List(vec!["admin".into(), "user".into()])],
            rate_limit_rps: 100.0,
            ..Default::default()
        };
        let summary = engine
            .run(
                &format!(
                    "GET /u/\u{a7}u\u{a7} HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                    addr.port()
                ),
                &cfg,
            )
            .await
            .unwrap();
        server.abort();
        assert_eq!(summary.results.len(), 2);
        assert_eq!(summary.sent, 2);
        assert!(summary.parse_errors.is_empty());
        let statuses: Vec<u16> = summary.results.iter().map(|r| r.metrics.status_code).collect();
        assert_ne!(statuses[0], statuses[1], "server should emit two different statuses");
    }
}
