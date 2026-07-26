#![allow(dead_code)]

use crate::history::models::HistoryEntry;

pub fn matches_search(entry: &HistoryEntry, query: &str, field: HistorySearchField) -> bool {
    let q = query.to_lowercase();
    let target = match field {
        HistorySearchField::Url => &entry.url,
        HistorySearchField::Host => &entry.host,
        HistorySearchField::Path => &entry.path,
        HistorySearchField::Method => &entry.method,
        HistorySearchField::Status if q.parse::<u16>().is_ok() => {
            return entry.status_code == q.parse::<u16>().unwrap()
        }
        HistorySearchField::Body => {
            return entry.request_body.to_lowercase().contains(&q)
                || entry.response_body.to_lowercase().contains(&q)
        }
        HistorySearchField::Tag => return entry.tags.to_lowercase().contains(&q),
        HistorySearchField::Status => return false,
    };
    target.to_lowercase().contains(&q)
}

pub fn search<'a>(
    entries: &'a [HistoryEntry],
    query: &str,
    field: HistorySearchField,
) -> Vec<&'a HistoryEntry> {
    entries
        .iter()
        .filter(|e| matches_search(e, query, field))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistorySearchField {
    Url,
    Host,
    Path,
    Method,
    Status,
    Body,
    Tag,
}
