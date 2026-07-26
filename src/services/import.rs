use uuid::Uuid;

use crate::db::repository::Repository;
use crate::models::manifest::ProjectBundle;

pub fn validate_bundle(bundle: &ProjectBundle) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if bundle.manifest.project_version.is_empty() {
        errors.push("Missing project_version in manifest".into());
    }
    if bundle.manifest.assessment_id.is_empty() {
        errors.push("Missing assessment_id in manifest".into());
    }
    if bundle.assessment.is_none() {
        errors.push("Missing assessment data".into());
    }
    if bundle.sections.is_empty() {
        errors.push("Project contains no workflow sections".into());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn import_project(repo: &Repository, bundle: &ProjectBundle) -> anyhow::Result<()> {
    validate_bundle(bundle)
        .map_err(|errs| anyhow::anyhow!("Validation failed: {}", errs.join("; ")))?;

    let assessment = bundle.assessment.as_ref().unwrap();
    repo.create_assessment(assessment)?;

    for section in &bundle.sections {
        repo.insert_section(section)?;
    }
    for activity in &bundle.activities {
        repo.insert_activity(activity)?;
    }
    for task in &bundle.tasks {
        repo.insert_task(task)?;
    }
    for ev in &bundle.evidence {
        repo.insert_evidence(ev)?;
    }
    for finding in &bundle.findings {
        repo.insert_finding(finding)?;
    }
    for link in &bundle.finding_evidence {
        if let (Ok(fid), Ok(eid)) = (
            Uuid::parse_str(&link.finding_id),
            Uuid::parse_str(&link.evidence_id),
        ) {
            let _ = repo.link_evidence(&fid, &eid);
        }
    }
    for req in &bundle.http_requests {
        repo.insert_http_request(req)?;
    }
    for resp in &bundle.http_responses {
        repo.insert_http_response(resp)?;
    }
    for txn in &bundle.http_transactions {
        repo.insert_http_transaction(txn)?;
    }
    if let Some(ref session) = bundle.session {
        if let Ok(aid) = Uuid::parse_str(&session.assessment_id) {
            let _ = repo.save_session(&crate::models::session::Session {
                id: Uuid::new_v4(),
                assessment_id: aid,
                current_screen: session.current_screen.clone(),
                last_opened_at: chrono::Utc::now(),
            });
        }
    }

    Ok(())
}
