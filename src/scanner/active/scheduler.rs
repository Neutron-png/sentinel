#![allow(dead_code)]

use crate::scanner::active::events::ScanEventBus;
use crate::scanner::active::models::{JobStatus, ScanJob, ScanTask};
use crate::scanner::active::queue::ScanQueue;

pub struct ScanScheduler {
    queue: ScanQueue,
    event_bus: ScanEventBus,
    jobs: Vec<ScanJob>,
    tasks: Vec<ScanTask>,
}

impl ScanScheduler {
    pub fn new(queue_capacity: usize) -> Self {
        Self {
            queue: ScanQueue::new(queue_capacity),
            event_bus: ScanEventBus::new(1024),
            jobs: Vec::new(),
            tasks: Vec::new(),
        }
    }

    pub fn add_job(&mut self, job: ScanJob) -> &ScanJob {
        self.jobs.push(job);
        self.jobs.last().unwrap()
    }

    pub fn add_tasks(&mut self, tasks: Vec<ScanTask>) -> usize {
        let mut count = 0;
        for task in tasks {
            self.tasks.push(task.clone());
            if self.queue.enqueue(task) {
                count += 1;
            }
        }
        count
    }

    pub fn find_job(&self, id: uuid::Uuid) -> Option<&ScanJob> {
        self.jobs.iter().find(|j| j.id == id)
    }
    pub fn find_job_mut(&mut self, id: uuid::Uuid) -> Option<&mut ScanJob> {
        self.jobs.iter_mut().find(|j| j.id == id)
    }

    pub fn update_job_status(&mut self, job_id: uuid::Uuid, status: JobStatus) {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
            job.status = status;
            match status {
                JobStatus::Completed => {
                    job.finished_at = Some(chrono::Utc::now());
                    job.progress = 1.0;
                    self.event_bus
                        .emit(super::events::ScanEvent::JobCompleted { job_id });
                }
                JobStatus::Running => {
                    job.started_at = Some(chrono::Utc::now());
                    self.event_bus
                        .emit(super::events::ScanEvent::JobStarted { job_id });
                }
                JobStatus::Cancelled => {
                    self.event_bus
                        .emit(super::events::ScanEvent::JobCancelled { job_id });
                }
                _ => {}
            }
        }
    }

    pub fn update_progress(&mut self, job_id: uuid::Uuid, progress: f64) {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
            job.progress = progress;
            self.event_bus
                .emit(super::events::ScanEvent::JobProgress { job_id, progress });
        }
    }

    pub fn next_task(&mut self) -> Option<ScanTask> {
        self.queue.dequeue()
    }
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
    pub fn event_bus(&self) -> ScanEventBus {
        self.event_bus.clone()
    }
    pub fn jobs(&self) -> &[ScanJob] {
        &self.jobs
    }
    pub fn cancel_job(&mut self, job_id: uuid::Uuid) -> usize {
        self.update_job_status(job_id, JobStatus::Cancelled);
        self.queue.cancel_job(job_id)
    }
}
