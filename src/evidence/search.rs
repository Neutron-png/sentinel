#![allow(dead_code)]

use crate::evidence::models::EvidenceItem;

pub fn search_by_tag<'a>(items: &'a [&'a EvidenceItem], tag: &str) -> Vec<&'a EvidenceItem> {
    items
        .iter()
        .copied()
        .filter(|e| e.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
        .collect()
}

pub fn search_by_type<'a>(
    items: &'a [&'a EvidenceItem],
    ev_type: super::models::EvidenceType,
) -> Vec<&'a EvidenceItem> {
    items
        .iter()
        .copied()
        .filter(|e| e.evidence_type == ev_type)
        .collect()
}

pub fn search_by_module<'a>(items: &'a [&'a EvidenceItem], module: &str) -> Vec<&'a EvidenceItem> {
    items
        .iter()
        .copied()
        .filter(|e| e.source_module.eq_ignore_ascii_case(module))
        .collect()
}

pub fn search_by_keyword<'a>(
    items: &'a [&'a EvidenceItem],
    keyword: &str,
) -> Vec<&'a EvidenceItem> {
    let kw = keyword.to_lowercase();
    items
        .iter()
        .copied()
        .filter(|e| {
            e.description.to_lowercase().contains(&kw)
                || e.tags.iter().any(|t| t.to_lowercase().contains(&kw))
        })
        .collect()
}
