#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

impl JobStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Running => "Running",
            Self::Paused => "Paused",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

impl TaskState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Queued => "Queued",
            Self::Running => "Running",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
            Self::TimedOut => "Timed Out",
        }
    }
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled | Self::TimedOut
        )
    }
}

#[derive(Debug, Clone)]
pub struct ScanJob {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub target: String,
    pub status: JobStatus,
    pub progress: f64,
    pub priority: u8,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl ScanJob {
    pub fn new(assessment_id: Uuid, target: &str, priority: u8) -> Self {
        Self {
            id: Uuid::new_v4(),
            assessment_id,
            target: target.to_string(),
            status: JobStatus::Pending,
            progress: 0.0,
            priority,
            created_at: Utc::now(),
            started_at: None,
            finished_at: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScanTask {
    pub id: Uuid,
    pub job_id: Uuid,
    pub endpoint: String,
    pub rule_id: String,
    pub state: TaskState,
    pub retries: u32,
    pub max_retries: u32,
    pub timeout_secs: u64,
    pub result: Option<String>,
}

impl ScanTask {
    pub fn new(
        job_id: Uuid,
        endpoint: &str,
        rule_id: &str,
        timeout_secs: u64,
        max_retries: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            job_id,
            endpoint: endpoint.to_string(),
            rule_id: rule_id.to_string(),
            state: TaskState::Pending,
            retries: 0,
            max_retries,
            timeout_secs,
            result: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub worker_count: usize,
    pub retry_max: u32,
    pub retry_delay_ms: u64,
    pub default_timeout_secs: u64,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            worker_count: 4,
            retry_max: 2,
            retry_delay_ms: 1000,
            default_timeout_secs: 30,
        }
    }
}
