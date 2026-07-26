#![allow(dead_code)]

use std::sync::Arc;
use uuid::Uuid;

use crate::network::client::HttpClient;
use crate::scanner::active::errors::ScanError;
use crate::scanner::active::events::ScanEventBus;
use crate::scanner::active::models::{JobStatus, ScanJob, ScanTask};
use crate::scanner::active::registry::{ActiveRuleRegistry, ActiveScanRule};
use crate::scanner::active::scheduler::ScanScheduler;
use crate::scanner::active::session::ScanSessionManager;

pub struct ScanManager {
    session: ScanSessionManager,
    scheduler: ScanScheduler,
    rule_registry: ActiveRuleRegistry,
    event_bus: ScanEventBus,
    client: Arc<HttpClient>,
}

impl ScanManager {
    pub fn new(client: Arc<HttpClient>) -> Self {
        Self {
            session: ScanSessionManager::new(50),
            scheduler: ScanScheduler::new(10000),
            rule_registry: ActiveRuleRegistry::new(),
            event_bus: ScanEventBus::new(1024),
            client,
        }
    }

    pub fn register_rule(&mut self, rule: Arc<dyn ActiveScanRule>) {
        self.rule_registry.register(rule);
    }

    pub fn create_job(
        &mut self,
        assessment_id: Uuid,
        target: &str,
        priority: u8,
    ) -> Result<&ScanJob, ScanError> {
        self.session.create_job(assessment_id, target, priority)
    }

    pub fn enqueue_tasks(
        &mut self,
        job_id: Uuid,
        tasks: Vec<ScanTask>,
    ) -> Result<usize, ScanError> {
        self.session
            .find_job(job_id)
            .ok_or_else(|| ScanError::NotFound(job_id.to_string()))?;
        Ok(self.scheduler.add_tasks(tasks))
    }

    pub fn start_job(&mut self, job_id: Uuid) -> Result<(), ScanError> {
        self.scheduler.update_job_status(job_id, JobStatus::Running);
        Ok(())
    }

    pub fn pause_job(&mut self, job_id: Uuid) -> Result<(), ScanError> {
        self.scheduler.update_job_status(job_id, JobStatus::Paused);
        Ok(())
    }

    pub fn resume_job(&mut self, job_id: Uuid) -> Result<(), ScanError> {
        self.scheduler.update_job_status(job_id, JobStatus::Running);
        self.event_bus
            .emit(super::events::ScanEvent::JobResumed { job_id });
        Ok(())
    }

    pub fn cancel_job(&mut self, job_id: Uuid) -> Result<(), ScanError> {
        self.session.cancel_job(job_id)?;
        self.scheduler
            .update_job_status(job_id, JobStatus::Cancelled);
        Ok(())
    }

    pub fn next_task(&mut self) -> Option<ScanTask> {
        self.scheduler.next_task()
    }
    pub fn jobs(&self) -> &[ScanJob] {
        self.session.all_jobs()
    }
    pub fn active_jobs(&self) -> Vec<&ScanJob> {
        self.session.active_jobs()
    }
    pub fn event_bus(&self) -> ScanEventBus {
        self.event_bus.clone()
    }
    pub fn pending_count(&self) -> usize {
        self.scheduler.pending_count()
    }
}
