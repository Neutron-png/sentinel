#![allow(dead_code)]

use uuid::Uuid;

use crate::db::repository::Repository;
use crate::history::models::HistoryEntry;

pub struct HistoryRepository<'a> {
    repo: &'a Repository,
}

impl<'a> HistoryRepository<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        Self { repo }
    }

    pub fn insert(&self, entry: &HistoryEntry) -> anyhow::Result<()> {
        self.repo.insert_history(entry)
    }

    pub fn get(&self, id: &Uuid) -> anyhow::Result<Option<HistoryEntry>> {
        self.repo.get_history(id)
    }

    pub fn list(&self, limit: usize) -> anyhow::Result<Vec<HistoryEntry>> {
        self.repo.list_history(limit)
    }

    pub fn list_by_protocol(
        &self,
        version: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<HistoryEntry>> {
        self.repo.list_history_by_protocol(version, limit)
    }

    pub fn list_by_connection(
        &self,
        connection_id: &Uuid,
        limit: usize,
    ) -> anyhow::Result<Vec<HistoryEntry>> {
        self.repo.list_history_by_connection(connection_id, limit)
    }

    pub fn delete(&self, id: &Uuid) -> anyhow::Result<()> {
        self.repo.delete_history(id)
    }

    pub fn clear(&self) -> anyhow::Result<()> {
        self.repo.clear_history()
    }
}
