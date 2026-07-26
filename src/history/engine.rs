#![allow(dead_code)]

use chrono::Utc;
use uuid::Uuid;

use crate::history::errors::HistoryError;
use crate::history::filter::{filter, HistoryFilter};
use crate::history::models::HistoryEntry;
use crate::history::repository::HistoryRepository;
use crate::history::search::{search, HistorySearchField};
use crate::history::sort::{sort, HistorySortField, SortDirection};

pub struct HistoryEngine<'a> {
    repo: HistoryRepository<'a>,
    cache: Vec<HistoryEntry>,
}

impl<'a> HistoryEngine<'a> {
    pub fn new(repo: &'a crate::db::repository::Repository, limit: usize) -> anyhow::Result<Self> {
        let hr = HistoryRepository::new(repo);
        let cache = hr.list(limit).unwrap_or_default();
        Ok(Self { repo: hr, cache })
    }

    pub fn record(&mut self, entry: HistoryEntry) -> anyhow::Result<()> {
        self.repo.insert(&entry)?;
        self.cache.push(entry);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_transaction(
        &mut self,
        method: &str,
        url: &str,
        host: &str,
        status_code: u16,
        duration_ms: u64,
        request_body: &str,
        response_body: &str,
    ) -> anyhow::Result<()> {
        let parsed =
            url::Url::parse(url).unwrap_or_else(|_| url::Url::parse("http://unknown/").unwrap());
        let entry = HistoryEntry {
            id: Uuid::new_v4(),
            transaction_id: None,
            timestamp: Utc::now(),
            method: method.to_string(),
            scheme: parsed.scheme().to_string(),
            host: host.to_string(),
            port: parsed.port().unwrap_or(443),
            path: parsed.path().to_string(),
            query: parsed.query().unwrap_or("").to_string(),
            url: url.to_string(),
            protocol: "HTTP/1.1".into(),
            status_code,
            request_size: request_body.len() as u64,
            response_size: response_body.len() as u64,
            duration_ms,
            tls_enabled: parsed.scheme() == "https",
            source: "proxy".into(),
            tags: String::new(),
            request_body: request_body.to_string(),
            response_body: response_body.to_string(),
            request_headers: String::new(),
            response_headers: String::new(),
        };
        self.record(entry)
    }

    pub fn list(&self) -> &[HistoryEntry] {
        &self.cache
    }

    pub fn get(&self, id: &Uuid) -> Option<&HistoryEntry> {
        self.cache.iter().find(|e| e.id == *id)
    }

    pub fn delete(&mut self, id: &Uuid) -> Result<(), HistoryError> {
        self.repo
            .delete(id)
            .map_err(|e| HistoryError::Storage(e.to_string()))?;
        self.cache.retain(|e| e.id != *id);
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), HistoryError> {
        self.repo
            .clear()
            .map_err(|e| HistoryError::Storage(e.to_string()))?;
        self.cache.clear();
        Ok(())
    }

    pub fn search(&self, query: &str, field: HistorySearchField) -> Vec<&HistoryEntry> {
        search(&self.cache, query, field)
    }

    pub fn filter(&self, f: &HistoryFilter) -> Vec<&HistoryEntry> {
        filter(&self.cache, f)
    }

    pub fn sort_by(&mut self, field: HistorySortField, direction: SortDirection) {
        sort(&mut self.cache, field, direction);
    }

    pub fn add_tag(&mut self, id: &Uuid, tag: &str) -> Result<(), HistoryError> {
        if let Some(entry) = self.cache.iter_mut().find(|e| e.id == *id) {
            if entry.tags.is_empty() {
                entry.tags = tag.to_string();
            } else if !entry.tags.contains(tag) {
                entry.tags = format!("{},{}", entry.tags, tag);
            }
        }
        Ok(())
    }

    pub fn remove_tag(&mut self, id: &Uuid, tag: &str) -> Result<(), HistoryError> {
        if let Some(entry) = self.cache.iter_mut().find(|e| e.id == *id) {
            let tags: Vec<&str> = entry
                .tags
                .split(',')
                .map(|t| t.trim())
                .filter(|t| *t != tag)
                .collect();
            entry.tags = tags.join(",");
        }
        Ok(())
    }
}
