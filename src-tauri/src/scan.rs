use std::sync::Arc;
use std::collections::HashSet;

use chrono::Utc;
use tauri::{Emitter, State};
use uuid::Uuid;

use sentinel::network::client::HttpClient;
use sentinel::models::finding::{Confidence, Finding, FindingStatus, Severity};
use sentinel::scanner::pipeline::executor::PipelineExecutor;
use sentinel::scanner::rules::active::path_traversal::rule::PathTraversalRule;
use sentinel::scanner::rules::active::sqli::rule::SqliRule;
use sentinel::scanner::rules::active::xss::rule::XssRule;
use sentinel::scanner::sdk::record::VerificationRecord;
use sentinel::scanner::sdk::result::{RuleResult, RuleStatus};
use sentinel::scanner::sdk::rule::ScanRule;

use super::{http_client_config, ProxyState};

/// Cancel handle for the scan in progress, if any. One scan at a time.
#[derive(Default)]
pub struct ScanControl(pub std::sync::Mutex<Option<tokio::sync::watch::Sender<bool>>>);

#[tauri::command]
pub fn cancel_scan(control: State<'_, ScanControl>) -> Result<(), String> {
    match control.0.lock().unwrap().as_ref() {
        Some(tx) => {
            let _ = tx.send(true);
            Ok(())
        }
        None => Err("No scan is running.".into()),
    }
}

/// Mark which other techniques flagged the same parameter, so the gate can
/// count a second technique as independent verification.
fn corroborate(results: &mut [RuleResult]) {
    let keys: Vec<(String, String, String)> = results
        .iter()
        .filter_map(|r| r.record.as_ref().map(|rec| (r.rule_id.clone(), rec.parameter.clone(), rec.technique.clone())))
        .collect();
    for r in results.iter_mut() {
        let rule_id = r.rule_id.clone();
        if let Some(rec) = r.record.as_mut() {
            let mut others: Vec<String> = keys
                .iter()
                .filter(|(rid, p, t)| *rid == rule_id && !p.is_empty() && *p == rec.parameter && *t != rec.technique)
                .map(|(_, _, t)| t.clone())
                .collect();
            others.sort();
            others.dedup();
            rec.corroborated_by = others;
        }
    }
}

#[derive(serde::Serialize, Clone)]
pub struct ScanFinding {
    pub rule_id: String,
    pub title: String,
    pub severity: String,
    pub confidence: String,
    pub status: String,
    pub description: String,
    pub recommendation: String,
    pub evidence: String,
}

#[derive(serde::Serialize, Clone)]
pub struct ScanProgress {
    pub rule_id: String,
    pub rule_name: String,
    pub stage: String,
    pub done: bool,
}

#[tauri::command]
pub async fn start_scan(
    app: tauri::AppHandle,
    proxy: State<'_, ProxyState>,
    control: State<'_, ScanControl>,
    assessment_id: String,
    target: String,
) -> Result<Vec<ScanFinding>, String> {
    let aid = Uuid::parse_str(&assessment_id).map_err(|e| e.to_string())?;
    let cfg = http_client_config(&proxy);
    let client = Arc::new(HttpClient::new(cfg).map_err(|e| format!("http client: {e}"))?);

    let rules: Vec<Box<dyn ScanRule>> = vec![
        Box::new(SqliRule::new()),
        Box::new(XssRule::new()),
        Box::new(PathTraversalRule::new()),
    ];

    let (cancel_tx, cancel_rx) = tokio::sync::watch::channel(false);
    {
        let mut slot = control.0.lock().unwrap();
        if slot.is_some() {
            return Err("A scan is already running. Cancel it or wait for it to finish.".into());
        }
        *slot = Some(cancel_tx.clone());
    }
    // Clear the cancel handle however this function returns.
    struct ClearOnDrop<'a>(&'a ScanControl);
    impl Drop for ClearOnDrop<'_> {
        fn drop(&mut self) {
            *self.0 .0.lock().unwrap() = None;
        }
    }
    let _clear = ClearOnDrop(&control);
    let mut all_findings: Vec<ScanFinding> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for rule in &rules {
        if *cancel_rx.borrow() {
            break;
        }
        let rid = rule.metadata().id.clone();
        let rname = rule.metadata().name.clone();
        let _ = app.emit("scan-progress", ScanProgress {
            rule_id: rid.clone(),
            rule_name: rname.clone(),
            stage: "running".into(),
            done: false,
        });

        let ex = PipelineExecutor::new();
        let (mut results, _progress) =
            ex.execute(rule.as_ref(), client.clone(), &target, cancel_tx.clone()).await;
        corroborate(&mut results);

        for r in results
            .iter()
            .filter(|r| r.status == RuleStatus::Confirmed || r.status == RuleStatus::Potential)
        {
            // Derive the gate from what was observed. The status the gate supports wins:
            // a rule cannot claim Confirmed past questions the engine never answered.
            let gate = r.record.as_ref().map(|rec| rec.derive_gate(r.confidence));
            let status = match &gate {
                Some(g) => VerificationRecord::gate_status(g, r.status),
                None => r.status,
            };
            let key = format!("{}|{}", r.rule_id, r.title);
            if !seen.insert(key) {
                continue;
            }
            let finding = ScanFinding {
                rule_id: r.rule_id.clone(),
                title: r.title.clone(),
                severity: r.severity.label().to_string(),
                confidence: r.confidence.label().to_string(),
                status: status.label().to_string(),
                description: r.description.clone(),
                recommendation: r.recommendation.clone(),
                evidence: r.evidence.clone(),
            };

            let severity =
                Severity::from_label(&finding.severity).unwrap_or(Severity::Informational);
            let confidence =
                Confidence::from_label(&finding.confidence).unwrap_or(Confidence::Medium);
            let row = Finding {
                id: Uuid::new_v4(),
                assessment_id: aid,
                task_id: aid, // linked to the assessment until workflow tasks integration
                title: finding.title.clone(),
                description: finding.description.clone(),
                severity,
                confidence,
                // Keep the engine's verdict: a result that passed verification is stored as Verified.
                status: if status == RuleStatus::Confirmed { FindingStatus::Verified } else { FindingStatus::Draft },
                impact: String::new(),
                recommendation: finding.recommendation.clone(),
                references: r.references.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            {
                let db = proxy.db.lock().unwrap();
                db.insert_finding(&row).map_err(|e| e.to_string())?;
                if let (Some(rec), Some(gate)) = (r.record.as_ref(), gate.as_ref()) {
                    // Stored as the record plus its derived gate, the shape the UI reads.
                    let mut json = serde_json::to_value(rec).map_err(|e| e.to_string())?;
                    json["gate"] = serde_json::to_value(gate).map_err(|e| e.to_string())?;
                    db.save_finding_record(&row.id, &json.to_string()).map_err(|e| e.to_string())?;
                }
            }
            let _ = app.emit("scan-finding", &finding);
            all_findings.push(finding);
        }

        let _ = app.emit("scan-progress", ScanProgress {
            rule_id: rid,
            rule_name: rname,
            stage: "complete".into(),
            done: true,
        });
    }

    if *cancel_rx.borrow() {
        return Err(format!(
            "Scan cancelled. {} result{} found before cancelling {} saved.",
            all_findings.len(),
            if all_findings.len() == 1 { "" } else { "s" },
            if all_findings.len() == 1 { "is" } else { "are" },
        ));
    }
    Ok(all_findings)
}
