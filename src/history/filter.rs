#![allow(dead_code)]

use crate::history::models::HistoryEntry;

pub fn filter<'a>(entries: &'a [HistoryEntry], f: &HistoryFilter) -> Vec<&'a HistoryEntry> {
    entries
        .iter()
        .filter(|e| {
            if let Some(ref methods) = f.methods {
                if !methods.iter().any(|m| m.eq_ignore_ascii_case(&e.method)) {
                    return false;
                }
            }
            if let Some(status) = f.status_code {
                if e.status_code != status {
                    return false;
                }
            }
            if let Some(ref proto) = f.protocol {
                if !e.protocol.eq_ignore_ascii_case(proto) {
                    return false;
                }
            }
            if let Some(ref host) = f.host {
                if !e.host.to_lowercase().contains(&host.to_lowercase()) {
                    return false;
                }
            }
            if let Some(ref ct) = f.content_type {
                if !e
                    .response_headers
                    .to_lowercase()
                    .contains(&ct.to_lowercase())
                {
                    return false;
                }
            }
            true
        })
        .collect()
}

#[derive(Debug, Clone, Default)]
pub struct HistoryFilter {
    pub methods: Option<Vec<String>>,
    pub status_code: Option<u16>,
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub content_type: Option<String>,
}
