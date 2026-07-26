#![allow(dead_code)]

use uuid::Uuid;

use crate::scanner::active::errors::ScanError;
use crate::scanner::active::models::{JobStatus, ScanJob};

#[derive(Default)]
pub struct ScanSessionManager {
    jobs: Vec<ScanJob>,
    max_jobs: usize,
}

impl ScanSessionManager {
    pub fn new(max_jobs: usize) -> Self {
        Self {
            jobs: Vec::new(),
            max_jobs,
        }
    }

    pub fn create_job(
        &mut self,
        assessment_id: Uuid,
        target: &str,
        priority: u8,
    ) -> Result<&ScanJob, ScanError> {
        if self.jobs.len() >= self.max_jobs {
            return Err(ScanError::JobLimit("Max jobs reached".into()));
        }
        let job = ScanJob::new(assessment_id, target, priority);
        self.jobs.push(job);
        Ok(self.jobs.last().unwrap())
    }

    pub fn all_jobs(&self) -> &[ScanJob] {
        &self.jobs
    }
    pub fn find_job(&self, id: Uuid) -> Option<&ScanJob> {
        self.jobs.iter().find(|j| j.id == id)
    }
    pub fn find_job_mut(&mut self, id: Uuid) -> Option<&mut ScanJob> {
        self.jobs.iter_mut().find(|j| j.id == id)
    }

    pub fn active_jobs(&self) -> Vec<&ScanJob> {
        self.jobs
            .iter()
            .filter(|j| {
                matches!(
                    j.status,
                    JobStatus::Pending | JobStatus::Running | JobStatus::Paused
                )
            })
            .collect()
    }

    pub fn cancel_job(&mut self, id: Uuid) -> Result<(), ScanError> {
        let job = self
            .jobs
            .iter_mut()
            .find(|j| j.id == id)
            .ok_or_else(|| ScanError::NotFound(id.to_string()))?;
        job.status = JobStatus::Cancelled;
        Ok(())
    }
}
