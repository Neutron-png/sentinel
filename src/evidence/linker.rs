#![allow(dead_code)]

use uuid::Uuid;

use crate::evidence::models::EvidenceItem;

pub fn link_to_finding(item: &mut EvidenceItem, finding_id: Uuid) {
    if !item.linked_findings.contains(&finding_id) {
        item.linked_findings.push(finding_id);
    }
}

pub fn link_to_transaction(item: &mut EvidenceItem, transaction_id: Uuid) {
    if !item.linked_transactions.contains(&transaction_id) {
        item.linked_transactions.push(transaction_id);
    }
}

pub fn unlink_finding(item: &mut EvidenceItem, finding_id: Uuid) {
    item.linked_findings.retain(|id| *id != finding_id);
}
