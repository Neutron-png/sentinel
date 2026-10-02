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
use sentinel::scanner::sdk::result::RuleStatus;
use sentinel::scanner::sdk::rule::ScanRule;

use super::{http_client_config, ProxyState};

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

    let (cancel_tx, _) = tokio::sync::watch::channel(false);
    let mut all_findings: Vec<ScanFinding> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for rule in &rules {
        let rid = rule.metadata().id.clone();
        let rname = rule.metadata().name.clone();
        let _ = app.emit("scan-progress", ScanProgress {
            rule_id: rid.clone(),
            rule_name: rname.clone(),
            stage: "running".into(),
            done: false,
        });

        let ex = PipelineExecutor::new();
        let (results, _progress) =
            ex.execute(rule.as_ref(), client.clone(), &target, cancel_tx.clone());

        for r in results
            .iter()
            .filter(|r| r.status == RuleStatus::Confirmed || r.status == RuleStatus::Potential)
        {
            let key = format!("{}|{}", r.rule_id, r.title);
            if !seen.insert(key) {
                continue;
            }
            let finding = ScanFinding {
                rule_id: r.rule_id.clone(),
                title: r.title.clone(),
                severity: r.severity.label().to_string(),
                confidence: r.confidence.label().to_string(),
                status: r.status.label().to_string(),
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
                status: FindingStatus::Draft,
                impact: String::new(),
                recommendation: finding.recommendation.clone(),
                references: r.references.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            proxy.db.lock().unwrap().insert_finding(&row).map_err(|e| e.to_string())?;
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

    Ok(all_findings)
}
