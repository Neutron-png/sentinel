#![allow(dead_code)]

use uuid::Uuid;

use crate::evidence::models::EvidenceItem;
use crate::evidence::storage::EvidenceStorage;

pub fn export_item(item: &EvidenceItem) -> String {
    serde_json::to_string_pretty(&serde_json::json!({
        "id": item.id.to_string(),
        "type": item.evidence_type.label(),
        "description": item.description,
        "tags": item.tags,
        "sha256": item.sha256_hash,
        "timestamp": item.timestamp.to_rfc3339(),
    }))
    .unwrap_or_default()
}

pub fn export_by_finding(storage: &EvidenceStorage, finding_id: Uuid) -> String {
    let items = storage.get_by_finding(finding_id);
    let arr: Vec<serde_json::Value> = items
        .iter()
        .map(|e| serde_json::from_str(&export_item(e)).unwrap_or_default())
        .collect();
    serde_json::to_string_pretty(&arr).unwrap_or_default()
}

pub fn export_by_assessment(storage: &EvidenceStorage, assessment_id: Uuid) -> String {
    let items = storage.get_by_assessment(assessment_id);
    let arr: Vec<serde_json::Value> = items
        .iter()
        .map(|e| serde_json::from_str(&export_item(e)).unwrap_or_default())
        .collect();
    serde_json::to_string_pretty(&arr).unwrap_or_default()
}
