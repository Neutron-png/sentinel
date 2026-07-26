use chrono::Utc;
use uuid::Uuid;

use crate::db::repository::Repository;
use crate::models::manifest::{FindingEvidenceLink, ProjectBundle, ProjectManifest, SessionData};

pub fn export_project(
    repo: &Repository,
    assessment_id: Uuid,
    assessment_name: &str,
    methodology: &str,
) -> anyhow::Result<String> {
    let sections = repo.load_sections(&assessment_id).unwrap_or_default();
    let mut activities = Vec::new();
    let mut tasks = Vec::new();
    let mut evidence = Vec::new();
    let mut findings = Vec::new();
    let mut finding_evidence_links = Vec::new();
    let mut http_requests = Vec::new();
    let mut http_responses = Vec::new();
    let mut http_transactions = Vec::new();

    for section in &sections {
        let acts = repo.load_activities(&section.id).unwrap_or_default();
        activities.extend(acts);
    }
    for activity in &activities {
        let ts = repo.load_tasks(&activity.id).unwrap_or_default();
        tasks.extend(ts);
    }
    for task in &tasks {
        let ev = repo.load_evidence(&task.id).unwrap_or_default();
        evidence.extend(ev);
        let f = repo.load_findings(&task.id).unwrap_or_default();
        findings.extend(f);
        let txns = repo.list_transactions(&task.id).unwrap_or_default();
        http_transactions.extend(txns);
    }
    for finding in &findings {
        let ids = repo
            .load_linked_evidence_ids(&finding.id)
            .unwrap_or_default();
        for eid in ids {
            finding_evidence_links.push(FindingEvidenceLink {
                finding_id: finding.id.to_string(),
                evidence_id: eid.to_string(),
            });
        }
    }
    for txn in &http_transactions {
        if let Ok(Some(req)) = repo.get_http_request(&txn.request_id) {
            http_requests.push(req);
        }
        if let Ok(Some(resp)) = repo.get_http_response(&txn.response_id) {
            http_responses.push(resp);
        }
    }

    let assessment = repo.get_assessment(&assessment_id).ok().flatten();
    let session = repo.load_session().ok().flatten();

    let bundle = ProjectBundle {
        manifest: ProjectManifest {
            project_version: "1.0".into(),
            sentinel_version: env!("CARGO_PKG_VERSION").into(),
            export_timestamp: Utc::now().to_rfc3339(),
            methodology: methodology.into(),
            assessment_id: assessment_id.to_string(),
            assessment_name: assessment_name.into(),
        },
        assessment,
        sections,
        activities,
        tasks,
        evidence,
        findings,
        finding_evidence: finding_evidence_links,
        http_requests,
        http_responses,
        http_transactions,
        session: session.map(|s| SessionData {
            assessment_id: s.assessment_id.to_string(),
            current_screen: s.current_screen,
            last_opened_at: s.last_opened_at.to_rfc3339(),
        }),
    };

    serde_json::to_string_pretty(&bundle).map_err(Into::into)
}
