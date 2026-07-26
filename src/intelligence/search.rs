#![allow(dead_code)]

use crate::intelligence::models::KnowledgeEntry;

pub fn search_by_cwe<'a>(entries: &'a [&'a KnowledgeEntry], cwe: &str) -> Vec<&'a KnowledgeEntry> {
    entries
        .iter()
        .filter(|e| e.cwe_id.eq_ignore_ascii_case(cwe))
        .copied()
        .collect()
}

pub fn search_by_owasp<'a>(
    entries: &'a [&'a KnowledgeEntry],
    category: &str,
) -> Vec<&'a KnowledgeEntry> {
    entries
        .iter()
        .filter(|e| {
            e.owasp_category
                .to_lowercase()
                .contains(&category.to_lowercase())
        })
        .copied()
        .collect()
}

pub fn search_by_keyword<'a>(
    entries: &'a [&'a KnowledgeEntry],
    kw: &str,
) -> Vec<&'a KnowledgeEntry> {
    let k = kw.to_lowercase();
    entries
        .iter()
        .filter(|e| {
            e.description.to_lowercase().contains(&k) || e.cwe_name.to_lowercase().contains(&k)
        })
        .copied()
        .collect()
}
