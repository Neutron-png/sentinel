#![allow(dead_code)]

use uuid::Uuid;

use crate::intelligence::models::KnowledgeEntry;

pub fn find_related(findings: &[(Uuid, &str)], _knowledge: &[KnowledgeEntry]) -> Vec<(Uuid, Uuid)> {
    let mut relations = Vec::new();
    for (i, (id1, cwe1)) in findings.iter().enumerate() {
        for (id2, cwe2) in findings.iter().skip(i + 1) {
            if cwe1 == cwe2 {
                relations.push((*id1, *id2));
            }
        }
    }
    relations
}
